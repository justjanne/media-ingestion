use ffmpeg_dev::sys as ffi;
use failure::bail;
use enum_primitive::*;
use std::marker::PhantomData;
use fraction::Fraction;

use crate::ffmpeg_api::enums::*;

pub struct AVFormatContext {
    base: *mut ffi::AVFormatContext,
}

impl<'a> AVFormatContext {
    pub fn new() -> Result<Self, failure::Error> {
        let base = unsafe { ffi::avformat_alloc_context() };
        if base.is_null() {
            bail!("avformat_alloc_context() failed");
        }
        Ok(AVFormatContext { base })
    }

    pub fn open_input(&mut self, path: &str) -> Result<(), failure::Error> {
        match unsafe {
            ffi::avformat_open_input(
                &mut self.base,
                std::ffi::CString::new(path)
                    .map_err(|_| failure::format_err!("Could not convert path to c string"))?
                    .as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        } {
            0 => Ok(()),
            _ => bail!("Could not open input")
        }
    }

    pub fn streams(&self) -> Vec<AVStream> {
        return unsafe {
            std::slice::from_raw_parts(
                (*self.base).streams,
                (*self.base).nb_streams as usize,
            )
        }
            .iter()
            .map(|stream| {
                AVStream::new(unsafe { (*stream).as_mut() }.expect("not null"), &self)
            })
            .collect();
    }

    pub fn read_frame(&/*TODO:mut*/ self, packet: &mut AVPacket) -> Result<(), failure::Error> {
        match unsafe { ffi::av_read_frame(self.base, packet.base) } {
            0 => Ok(()),
            errno => Err(failure::format_err!("Error while decoding frame: {}", errno))
        }
    }
}

impl Drop for AVFormatContext {
    fn drop(&mut self) {
        unsafe { ffi::avformat_free_context(self.base) }
    }
}

pub struct AVBuffer {
    base: *mut u8,
    size: usize,
}

impl AVBuffer {
    pub fn new(size: usize) -> Result<Self, failure::Error> {
        let base = unsafe { ffi::av_malloc(size) } as *mut u8;
        if base.is_null() {
            bail!("av_malloc() failed");
        }
        Ok(AVBuffer { base, size })
    }

    pub fn empty() -> Self {
        AVBuffer { base: std::ptr::null_mut(), size: 0 }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.base, self.size)
        }
    }

    pub fn data_mut(&mut self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.base, self.size)
        }
    }
}

pub struct AVPacket {
    base: *mut ffi::AVPacket,
}

impl AVPacket {
    pub fn new() -> Result<Self, failure::Error> {
        let base = unsafe { ffi::av_packet_alloc() };
        if base.is_null() {
            bail!("av_packet_alloc() failed");
        }
        Ok(AVPacket { base })
    }

    pub fn pts(&self) -> i64 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.pts
    }

    pub fn dts(&self) -> i64 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.dts
    }

    pub fn stream_index(&self) -> i32 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.stream_index
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

impl AVFrame {
    pub fn new() -> Result<Self, failure::Error> {
        let base = unsafe { ffi::av_frame_alloc() };
        if base.is_null() {
            bail!("avformat_alloc_frame() failed");
        }
        Ok(AVFrame { base, buffer: AVBuffer::empty() })
    }

    pub fn init(&mut self, width: i32, height: i32, format: AVPixelFormat) -> Result<(), failure::Error> {
        let mut base = unsafe { self.base.as_mut() }.expect("not null");

        base.width = width;
        base.height = height;
        base.format = format as ffi::AVPixelFormat;

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

    pub fn width(&self) -> i32 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.width
    }

    pub fn height(&self) -> i32 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.height
    }

    pub fn format(&self) -> AVPixelFormat {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        AVPixelFormat::from_i32(base.format)
            .unwrap_or(AVPixelFormat::NONE)
    }

    pub fn size(&self) -> usize {
        unsafe {
            ffi::avpicture_get_size(self.format() as ffi::AVPixelFormat, self.width(), self.height()) as usize
        }
    }

    pub fn key_frame(&self) -> bool {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.key_frame != 0
    }

    pub fn pts(&self) -> i64 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.pts
    }

    pub fn coded_picture_number(&self) -> i32 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.coded_picture_number
    }

    pub fn display_picture_number(&self) -> i32 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.display_picture_number
    }

    pub fn linesize(&self) -> &[i32] {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        &base.linesize
    }

    pub fn data_ptr(&self) -> *const *const u8 {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        base.data.as_ptr() as *const *const u8
    }

    pub fn data_mut_ptr(&mut self) -> *mut *mut u8 {
        let base = unsafe { self.base.as_mut() }.expect("not null");

        base.data.as_mut_ptr() as *mut *mut u8
    }

    pub fn data(&self, index: usize) -> &[u8] {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        unsafe {
            std::slice::from_raw_parts(base.data[index], self.size())
        }
    }

    pub fn data_mut(&mut self, index: usize) -> &mut [u8] {
        let base = unsafe { self.base.as_mut() }.expect("not null");

        unsafe {
            std::slice::from_raw_parts_mut(base.data[index], self.size())
        }
    }
}

impl Drop for AVFrame {
    fn drop(&mut self) {
        unsafe { ffi::av_frame_free(&mut self.base) }
    }
}

pub struct AVStream<'a> {
    base: &'a mut ffi::AVStream,
    phantom: PhantomData<&'a AVFormatContext>,
}

impl<'a> AVStream<'a> {
    fn new(base: &'a mut ffi::AVStream, _: &'a AVFormatContext) -> Self {
        return AVStream { base, phantom: PhantomData };
    }

    pub fn index(self: &AVStream<'a>) -> i32 {
        self.base.index
    }

    pub fn time_base(self: &AVStream<'a>) -> Fraction {
        Fraction::new(
            self.base.time_base.num as u32,
            self.base.time_base.den as u32,
        )
    }

    pub fn timestamp(self: &AVStream<'a>, timestamp: i64) -> std::time::Duration {
        std::time::Duration::from_millis(
            1000 *
                timestamp as u64 *
                self.base.time_base.num as u64 /
                self.base.time_base.den as u64
        )
    }

    pub fn duration(&self) -> std::time::Duration {
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

    pub fn codec_parameters(&self) -> AVCodecParameters {
        AVCodecParameters::new(unsafe { self.base.codecpar.as_mut() }.expect("not null"), self)
    }
}

pub struct AVCodecParameters<'a> {
    base: &'a mut ffi::AVCodecParameters,
    phantom: PhantomData<&'a AVStream<'a>>,
}

impl<'a> AVCodecParameters<'a> {
    fn new(base: &'a mut ffi::AVCodecParameters, _: &'a AVStream) -> Self {
        return AVCodecParameters { base, phantom: PhantomData };
    }

    pub fn codec_type(&self) -> AVMediaType {
        AVMediaType::from_i32(self.base.codec_type).unwrap_or(AVMediaType::Unknown)
    }

    pub fn codec_id(&self) -> Option<AVCodecID> {
        AVCodecID::from_u32(self.base.codec_id)
    }

    pub fn find_decoder(&self) -> AVCodec {
        AVCodec::new(
            unsafe { ffi::avcodec_find_decoder(self.base.codec_id).as_mut() }.expect("Decoder not found"),
            self,
        )
    }
}

pub struct AVCodec<'a> {
    base: &'a mut ffi::AVCodec,
    phantom: PhantomData<&'a AVCodecParameters<'a>>,
}

impl<'a> AVCodec<'a> {
    fn new(base: &'a mut ffi::AVCodec, _: &'a AVCodecParameters) -> Self {
        return AVCodec { base, phantom: PhantomData };
    }

    pub fn name(self: &AVCodec<'a>) -> std::string::String {
        String::from(unsafe { std::ffi::CStr::from_ptr(self.base.name) }.to_str().unwrap())
    }
}

pub struct AVCodecContext {
    base: *mut ffi::AVCodecContext,
}

impl AVCodecContext {
    pub fn new(codec: &AVCodec) -> Result<Self, failure::Error> {
        let base = unsafe { ffi::avcodec_alloc_context3(codec.base) };
        if base.is_null() {
            bail!("avcodec_alloc_context3() failed");
        }
        Ok(AVCodecContext { base })
    }

    pub fn in_packet(&mut self, packet: &mut AVPacket) -> Result<(), failure::Error> {
        match unsafe { ffi::avcodec_send_packet(self.base, packet.base) } {
            0 => Ok(()),
            errno => Err(failure::format_err!("Error while loading paclet: {}", errno))
        }
    }

    pub fn out_frame(&mut self, frame: &mut AVFrame) -> Result<(), failure::Error> {
        match unsafe { ffi::avcodec_receive_frame(self.base, frame.base) } {
            0 => Ok(()),
            errno => Err(failure::format_err!("Error while decoding frame: {}", errno))
        }
    }

    pub fn skip_loop_filter(&self) -> Option<AVDiscard> {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        AVDiscard::from_i32(base.skip_loop_filter)
    }

    pub fn set_skip_loop_filter(&mut self, value: AVDiscard) {
        let base = unsafe { self.base.as_mut() }.expect("not null");

        base.skip_loop_filter = value as ffi::AVDiscard
    }

    pub fn skip_idct(&self) -> Option<AVDiscard> {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        AVDiscard::from_i32(base.skip_idct)
    }

    pub fn set_skip_idct(&mut self, value: AVDiscard) {
        let base = unsafe { self.base.as_mut() }.expect("not null");

        base.skip_idct = value as ffi::AVDiscard
    }

    pub fn skip_frame(&self) -> Option<AVDiscard> {
        let base = unsafe { self.base.as_ref() }.expect("not null");

        AVDiscard::from_i32(base.skip_frame)
    }

    pub fn set_skip_frame(&mut self, value: AVDiscard) {
        let base = unsafe { self.base.as_mut() }.expect("not null");

        base.skip_frame = value as ffi::AVDiscard
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

impl SwsContext {
    pub fn new() -> Self {
        SwsContext { base: std::ptr::null_mut() }
    }

    pub fn reinit(&mut self, source: &AVFrame, target: &AVFrame, scaler: SwsScaler) -> Result<(), failure::Error> {
        let base = unsafe {
            ffi::sws_getCachedContext(
                self.base,
                source.width(),
                source.height(),
                source.format() as ffi::AVPixelFormat,
                target.width(),
                target.height(),
                target.format() as ffi::AVPixelFormat,
                scaler as i32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };
        if base.is_null() {
            bail!("sws_getCachedContext() failed");
        }
        self.base = base;

        Ok(())
    }

    pub fn scale(&self, source: &AVFrame, target: &mut AVFrame) -> i32 {
        self.scale_slice(source, target, 0, source.height())
    }

    pub fn scale_slice(&self, source: &AVFrame, target: &mut AVFrame, slice_from: i32, slice_to: i32) -> i32 {
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