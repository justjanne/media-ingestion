use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use ffmpeg_dev::sys as ffi;
use fraction::Fraction;
use num_traits::FromPrimitive;
use media_time::MediaTimeError;
use thiserror::Error;

use crate::enums::*;
use crate::err::AVError;

#[derive(Error, Debug)]
pub enum AVAllocError {
    #[error("Allocating a new {0} failed")]
    AllocFailed(String)
}

#[derive(Error, Debug)]
pub enum StringError {
    #[error("String is unexpectedly null")]
    NullError,
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error)
}

fn native_string(ptr: *const std::os::raw::c_char) -> Result<String, StringError> {
    if ptr.is_null() {
        Err(StringError::NullError)
    } else {
        Ok(String::from(
            unsafe { std::ffi::CStr::from_ptr(ptr) }
                .to_str()?,
        ))
    }
}

pub struct AVFormatContext {
    base: *mut ffi::AVFormatContext,
}

#[derive(Error, Debug)]
pub enum AVFormatContextError {
    #[error(transparent)]
    AllocFailed(#[from] AVAllocError),
    #[error("Path {0} is invalid")]
    PathInvalid(PathBuf),
    #[error("Path {0} contains null byte")]
    PathContainsNull(PathBuf, #[source] std::ffi::NulError),
    #[error("Opening media file {0} failed")]
    OpenInputFailed(PathBuf, #[source] AVError)
}

impl AVFormatContext {
    pub fn new() -> Result<Self, AVAllocError> {
        let base = unsafe { ffi::avformat_alloc_context() };
        return if base.is_null() {
            Err(AVAllocError::AllocFailed("AVFormatContext".to_string()))
        } else {
            Ok(AVFormatContext { base })
        }
    }

    pub fn open_input(&mut self, path: &Path) -> Result<(), AVFormatContextError> {
        let pathname = path
            .to_str()
            .ok_or(AVFormatContextError::PathInvalid(path.to_path_buf()))?;
        let pathname = std::ffi::CString::new(pathname)
            .map_err(|err| AVFormatContextError::PathContainsNull(path.to_path_buf(), err))?;

        AVError::from_errno(unsafe {
            ffi::avformat_open_input(
                &mut self.base,
                pathname.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        }).map_err(|err| AVFormatContextError::OpenInputFailed(path.to_path_buf(), err))
    }

    pub fn input_format(&self) -> Result<AVInputFormat, AVInputFormatError> {
        let base: &mut ffi::AVInputFormat = unsafe { (*self.base).iformat.as_mut() }
            .ok_or(AVInputFormatError::Invalid)?;

        Ok(AVInputFormat::new(base))
    }

    pub fn streams(&self) -> impl Iterator<Item = AVStream> {
        unsafe {
            std::slice::from_raw_parts((*self.base).streams, (*self.base).nb_streams as usize)
        }
        .iter()
        .filter_map(|stream: &*mut ffi::AVStream| unsafe { (*stream).as_mut() })
        .map(|stream| AVStream::new(stream))
    }

    pub fn read_frame(&mut self, packet: &mut AVPacket) -> Result<(), AVFrameError> {
        AVError::from_errno(unsafe { ffi::av_read_frame(self.base, packet.base) })
            .map_err(|err| AVFrameError::DecodingFailed(packet.stream_index(), packet.pts(), err))
    }

    pub fn duration(&self) -> Result<media_time::MediaTime, MediaTimeError> {
        media_time::MediaTime::from_rational(
            unsafe { (*self.base).duration },
            &Fraction::new(1 as u64, ffi::AV_TIME_BASE as u64),
        )
    }
}

impl Drop for AVFormatContext {
    fn drop(&mut self) {
        unsafe { ffi::avformat_free_context(self.base) }
    }
}

pub struct AVInputFormat<'a> {
    base: &'a mut ffi::AVInputFormat,
}

#[derive(Error, Debug)]
pub enum AVInputFormatError {
    #[error("AVInputFormat not valid")]
    Invalid,
    #[error("Field {0} is invalid")]
    FieldInaccessible(String, #[source] StringError),
    #[error("Mime type invalid: codec {0} in container {1}")]
    InvalidMime(String, String),
}

impl<'a> AVInputFormat<'a> {
    fn new(base: &'a mut ffi::AVInputFormat) -> Self {
        return AVInputFormat { base };
    }

    pub fn long_name(&self) -> Result<String, AVInputFormatError> {
        native_string(self.base.long_name)
            .map_err(|err| AVInputFormatError::FieldInaccessible("long_name".to_string(), err))
    }

    pub fn name(&self) -> Result<String, AVInputFormatError> {
        native_string(self.base.name)
            .map_err(|err| AVInputFormatError::FieldInaccessible("name".to_string(), err))
    }

    pub fn determine_mime(&self, stream_codec: impl AsRef<str>) -> Result<&str, AVInputFormatError> {
        let containers = self.name()?;
        let stream_codec = stream_codec.as_ref();

        for container in containers.split(",") {
            match (container, stream_codec) {
                ("mp4", "h264") | ("mp4", "hevc") => return Ok("video/mp4"),
                ("matroska", "h264") | ("matroska", "hevc") => return Ok("video/x-matroska"),
                ("webm", "vp8") | ("webm", "vp9") | ("webm", "av1") => return Ok("video/webm"),
                _ => {}
            }
        }

        Err(AVInputFormatError::InvalidMime(stream_codec.to_string(), containers.to_string()))
    }
}

pub struct AVBuffer {
    base: *mut u8,
    size: usize,
}

impl AVBuffer {
    pub fn new(size: usize) -> Result<Self, AVAllocError> {
        let base = unsafe { ffi::av_malloc(size) } as *mut u8;
        return if base.is_null() {
            Err(AVAllocError::AllocFailed("AVBufferContext".to_string()))
        } else {
            Ok(AVBuffer { base, size })
        }
    }

    pub fn empty() -> Self {
        AVBuffer {
            base: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.base, self.size) }
    }

    pub fn data_mut(&mut self) -> &[u8] {
        unsafe { std::slice::from_raw_parts_mut(self.base, self.size) }
    }
}

pub struct AVPacket {
    base: *mut ffi::AVPacket,
}

#[derive(Error, Debug)]
pub enum AVPacketError {
    #[error(transparent)]
    AllocFailed(#[from] AVAllocError),
}

impl AVPacket {
    pub fn new() -> Result<Self, AVAllocError> {
        let base = unsafe { ffi::av_packet_alloc() };
        return if base.is_null() {
            Err(AVAllocError::AllocFailed("AVPacket".to_string()))
        } else {
            Ok(AVPacket { base })
        }
    }

    fn as_ref(&self) -> &ffi::AVPacket {
        unsafe { self.base.as_ref() }.unwrap_or_else(|| panic!("AVPacket base unexpectedly null"))
    }

    pub fn pts(&self) -> i64 {
        self.as_ref().pts
    }

    pub fn dts(&self) -> i64 {
        self.as_ref().dts
    }

    pub fn stream_index(&self) -> i32 {
        self.as_ref().stream_index
    }
}

impl Drop for AVPacket {
    fn drop(&mut self) {
        unsafe { ffi::av_packet_free(&mut self.base) }
    }
}

pub struct AVFrame {
    base: *mut ffi::AVFrame,
    buffer: AVBuffer,
}

#[derive(Error, Debug)]
pub enum AVFrameError {
    #[error(transparent)]
    AllocFailed(#[from] AVAllocError),
    #[error("Decoding a frame from packet of stream {0} at timestamp {1} failed")]
    DecodingFailed(i32, i64, #[source] AVError)
}

impl AVFrame {
    pub fn new() -> Result<Self, AVAllocError> {
        let base = unsafe { ffi::av_frame_alloc() };
        return if base.is_null() {
            Err(AVAllocError::AllocFailed("AVFrame".to_string()))
        } else {
            Ok(AVFrame { base, buffer: AVBuffer::empty(), })
        }
    }

    pub fn init(&mut self, width: i32, height: i32, format: AVPixelFormat) -> Result<(), AVFrameError> {
        self.as_mut().width = width;
        self.as_mut().height = height;
        self.as_mut().format = format as ffi::AVPixelFormat;

        self.buffer = AVBuffer::new(self.size())?;

        unsafe {
            ffi::avpicture_fill(
                self.base as *mut ffi::AVPicture,
                self.buffer.base as *mut u8,
                self.format() as ffi::AVPixelFormat,
                self.width(),
                self.height(),
            )
        };

        Ok(())
    }

    fn as_ref(&self) -> &ffi::AVFrame {
        unsafe { self.base.as_ref() }.unwrap_or_else(|| panic!("AVFrame base unexpectedly null"))
    }

    fn as_mut(&mut self) -> &mut ffi::AVFrame {
        unsafe { self.base.as_mut() }.unwrap_or_else(|| panic!("AVFrame base unexpectedly null"))
    }

    pub fn width(&self) -> i32 {
        self.as_ref().width
    }

    pub fn height(&self) -> i32 {
        self.as_ref().height
    }

    pub fn format(&self) -> AVPixelFormat {
        AVPixelFormat::from_i32(self.as_ref().format).unwrap_or(AVPixelFormat::NONE)
    }

    pub fn size(&self) -> usize {
        unsafe {
            ffi::avpicture_get_size(
                self.format() as ffi::AVPixelFormat,
                self.width(),
                self.height(),
            ) as usize
        }
    }

    pub fn key_frame(&self) -> bool {
        self.as_ref().key_frame != 0
    }

    pub fn pts(&self) -> i64 {
        self.as_ref().pts
    }

    pub fn coded_picture_number(&self) -> i32 {
        self.as_ref().coded_picture_number
    }

    pub fn display_picture_number(&self) -> i32 {
        self.as_ref().display_picture_number
    }

    pub fn linesize(&self) -> &[i32] {
        &self.as_ref().linesize
    }

    pub fn data_ptr(&self) -> *const *const u8 {
        self.as_ref().data.as_ptr() as *const *const u8
    }

    pub fn data_mut_ptr(&mut self) -> *mut *mut u8 {
        self.as_mut().data.as_mut_ptr() as *mut *mut u8
    }

    pub fn data(&self, index: usize) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ref().data[index], self.size()) }
    }

    pub fn data_mut(&mut self, index: usize) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut().data[index], self.size()) }
    }
}

impl Drop for AVFrame {
    fn drop(&mut self) {
        unsafe { ffi::av_frame_free(&mut self.base) }
    }
}

pub struct AVStream<'a> {
    base: &'a mut ffi::AVStream,
}

impl<'a> AVStream<'a> {
    fn new(base: &'a mut ffi::AVStream) -> Self {
        return AVStream { base };
    }

    pub fn index(&self) -> i32 {
        self.base.index
    }

    pub fn time_base(&self) -> Fraction {
        Fraction::new(
            self.base.time_base.num as u32,
            self.base.time_base.den as u32,
        )
    }

    pub fn timestamp(&self, timestamp: i64) -> Result<media_time::MediaTime, MediaTimeError> {
        media_time::MediaTime::from_rational(timestamp, &self.time_base())
    }

    pub fn duration(&self) -> Result<media_time::MediaTime, MediaTimeError> {
        self.timestamp(self.base.duration)
    }

    pub fn frame_count(&self) -> i64 {
        self.base.nb_frames
    }

    pub fn discard(&self) -> Option<AVDiscard> {
        AVDiscard::from_i32(self.base.discard)
    }

    pub fn set_discard(&mut self, value: AVDiscard) {
        self.base.discard = value as ffi::AVDiscard;
    }

    pub fn sample_aspect_ratio(&self) -> Fraction {
        Fraction::new(
            self.base.sample_aspect_ratio.num as u32,
            self.base.sample_aspect_ratio.den as u32,
        )
    }

    pub fn display_aspect_ratio(&self) -> Fraction {
        Fraction::new(
            self.base.display_aspect_ratio.num as u32,
            self.base.display_aspect_ratio.den as u32,
        )
    }

    pub fn codec_parameters(&self) -> Result<AVCodecParameters, AVCodecParametersError> {
        Ok(AVCodecParameters::new(
            unsafe { self.base.codecpar.as_mut() }
                .ok_or(AVCodecParametersError::Invalid(self.index()))?,
            self,
        ))
    }
}

pub struct AVCodecParameters<'a> {
    base: &'a mut ffi::AVCodecParameters,
    phantom: PhantomData<&'a AVStream<'a>>,
}

#[derive(Error, Debug)]
pub enum AVCodecParametersError {
    #[error("AVCodecParameters not valid for stream {0}")]
    Invalid(i32),
}

impl<'a> AVCodecParameters<'a> {
    fn new(base: &'a mut ffi::AVCodecParameters, _: &'a AVStream) -> Self {
        return AVCodecParameters {
            base,
            phantom: PhantomData,
        };
    }

    pub fn codec_type(&self) -> AVMediaType {
        AVMediaType::from_i32(self.base.codec_type).unwrap_or(AVMediaType::Unknown)
    }

    pub fn codec_id(&self) -> Option<AVCodecID> {
        AVCodecID::from_u32(self.base.codec_id)
    }

    pub fn bit_rate(&self) -> i64 {
        self.base.bit_rate
    }

    pub fn find_decoder(&self) -> Result<AVCodec, AVCodecError> {
        Ok(AVCodec::new(
            unsafe { ffi::avcodec_find_decoder(self.base.codec_id).as_mut() }
                .ok_or(AVCodecError::Invalid)?,
            self,
        ))
    }
}

pub struct AVCodec<'a> {
    base: &'a mut ffi::AVCodec,
    phantom: PhantomData<&'a AVCodecParameters<'a>>,
}

#[derive(Error, Debug)]
pub enum AVCodecError {
    #[error("AVCodec not valid")]
    Invalid,
    #[error("Field {0} is invalid")]
    FieldInaccessible(String, #[source] StringError),
}

impl<'a> AVCodec<'a> {
    fn new(base: &'a mut ffi::AVCodec, _: &'a AVCodecParameters) -> Self {
        return AVCodec {
            base,
            phantom: PhantomData,
        };
    }

    pub fn name(&self) -> Result<String, AVCodecError> {
        native_string(self.base.name)
            .map_err(|err| AVCodecError::FieldInaccessible("name".to_string(), err))
    }
}

pub struct AVCodecContext {
    base: *mut ffi::AVCodecContext,
}

#[derive(Error, Debug)]
pub enum AVCodecContextError {
    #[error(transparent)]
    AllocFailed(#[from] AVAllocError),
    #[error("Field {0} is invalid")]
    FieldInaccessible(String, #[source] StringError),
    #[error("Error decoding packet")]
    PacketError(#[source] AVError),
    #[error("Error decoding frame")]
    FrameError(#[source] AVError),
}

impl AVCodecContext {
    pub fn new(codec: &AVCodec) -> Result<Self, AVAllocError> {
        let base = unsafe { ffi::avcodec_alloc_context3(codec.base) };
        if base.is_null() {
            Err(AVAllocError::AllocFailed("AVCodecContext".to_string()))
        } else {
            Ok(AVCodecContext { base })
        }
    }

    pub fn in_packet(&mut self, packet: &mut AVPacket) -> Result<(), AVCodecContextError> {
        AVError::from_errno(unsafe { ffi::avcodec_send_packet(self.base, packet.base) })
            .map_err(|err| AVCodecContextError::PacketError(err))
    }

    pub fn out_frame(&mut self, frame: &mut AVFrame) -> Result<(), AVCodecContextError> {
        AVError::from_errno( unsafe { ffi::avcodec_receive_frame(self.base, frame.base) })
            .map_err(|err| AVCodecContextError::FrameError(err))
    }

    fn as_ref(&self) -> &ffi::AVCodecContext {
        unsafe { self.base.as_ref() }
            .unwrap_or_else(|| panic!("AVCodecContext base unexpectedly null"))
    }

    fn as_mut(&mut self) -> &mut ffi::AVCodecContext {
        unsafe { self.base.as_mut() }
            .unwrap_or_else(|| panic!("AVCodecContext base unexpectedly null"))
    }

    pub fn skip_loop_filter(&self) -> Option<AVDiscard> {
        AVDiscard::from_i32(self.as_ref().skip_loop_filter)
    }

    pub fn set_skip_loop_filter(&mut self, value: AVDiscard) {
        self.as_mut().skip_loop_filter = value as ffi::AVDiscard
    }

    pub fn skip_idct(&self) -> Option<AVDiscard> {
        AVDiscard::from_i32(self.as_ref().skip_idct)
    }

    pub fn set_skip_idct(&mut self, value: AVDiscard) {
        self.as_mut().skip_idct = value as ffi::AVDiscard
    }

    pub fn skip_frame(&self) -> Option<AVDiscard> {
        AVDiscard::from_i32(self.as_ref().skip_frame)
    }

    pub fn set_skip_frame(&mut self, value: AVDiscard) {
        self.as_mut().skip_frame = value as ffi::AVDiscard
    }

    pub fn set_parameters(&mut self, params: &AVCodecParameters) {
        unsafe {
            ffi::avcodec_parameters_to_context(self.base, params.base);
        }
    }

    pub fn open(&mut self, codec: &AVCodec) {
        unsafe {
            ffi::avcodec_open2(self.base, codec.base, std::ptr::null_mut());
        }
    }
}

impl Drop for AVCodecContext {
    fn drop(&mut self) {
        unsafe { ffi::avcodec_free_context(&mut self.base) }
    }
}

pub struct SwsContext {
    base: *mut ffi::SwsContext,
}

#[derive(Error, Debug)]
pub enum SwsContextError {
    #[error(transparent)]
    AllocFailed(#[from] AVAllocError),
}

impl SwsContext {
    pub fn new() -> Self {
        SwsContext {
            base: std::ptr::null_mut(),
        }
    }

    pub fn reinit(
        &mut self,
        source: &AVFrame,
        target: &AVFrame,
        scaler: SwsScaler,
        flags: SwsFlags,
    ) -> Result<(), AVAllocError> {
        let base = unsafe {
            ffi::sws_getCachedContext(
                self.base,
                source.width(),
                source.height(),
                source.format() as ffi::AVPixelFormat,
                target.width(),
                target.height(),
                target.format() as ffi::AVPixelFormat,
                scaler as std::os::raw::c_int | flags.bits() as std::os::raw::c_int,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };

        if base.is_null() {
            Err(AVAllocError::AllocFailed("SwsContext".to_string()))
        } else {
            self.base = base;
            Ok(())
        }
    }

    pub fn scale(&self, source: &AVFrame, target: &mut AVFrame) -> i32 {
        self.scale_slice(source, target, 0, source.height())
    }

    pub fn scale_slice(
        &self,
        source: &AVFrame,
        target: &mut AVFrame,
        slice_from: i32,
        slice_to: i32,
    ) -> i32 {
        unsafe {
            ffi::sws_scale(
                self.base,
                source.data_ptr(),
                source.linesize().as_ptr(),
                slice_from,
                slice_to,
                target.data_mut_ptr(),
                target.linesize().as_ptr(),
            )
        }
    }
}

impl Drop for SwsContext {
    fn drop(&mut self) {
        unsafe { ffi::sws_freeContext(self.base) }
    }
}
