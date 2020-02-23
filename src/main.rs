use ffmpeg_dev::sys as ffi;

pub(crate) mod ffmpeg_api;

use crate::ffmpeg_api::enums::*;
use crate::ffmpeg_api::api::*;

fn main() -> Result<(), std::io::Error> {
    let mut before = std::time::SystemTime::now();

    let path = "/home/janne/Workspace/justflix/data/video.mp4";

    let mut avformat_context = AVFormatContext::new().unwrap_or_else(|error| {
        panic!("Could not allocate a context to process the video: {:?}", error)
    });
    avformat_context.open_input(path).unwrap_or_else(|error| {
        panic!("Could not open video input: {:?}", error)
    });

    for mut stream in avformat_context.streams() {
        stream.set_discard(AVDiscard::NonKey);

        let codec_parameters = stream.codec_parameters();
        let local_codec = codec_parameters.find_decoder();

        println!(
            "Stream #{}, type: {:#?}, codec: {:#?}",
            stream.index(),
            codec_parameters.codec_type(),
            local_codec.name()
        );

        let mut output_frame = AVFrame::new().unwrap_or_else(|error| {
            panic!("Could not create output frame: {:?}", error)
        });
        output_frame.init(160, 90, AVPixelFormat::RGB24).unwrap_or_else(|error| {
            panic!("Could not init output frame: {:?}", error)
        });

        match codec_parameters.codec_type() {
            AVMediaType::Video => {

                // TODO: HERE BE DRAGONS

                let avc_ctx: &mut ffi::AVCodecContext = unsafe {
                    ffi::avcodec_alloc_context3(local_codec.as_ref()).as_mut()
                }.expect("not null");

                avc_ctx.skip_loop_filter = ffi::AVDiscard_AVDISCARD_NONKEY;
                avc_ctx.skip_idct = ffi::AVDiscard_AVDISCARD_NONKEY;
                avc_ctx.skip_frame = ffi::AVDiscard_AVDISCARD_NONKEY;

                unsafe {
                    ffi::avcodec_parameters_to_context(avc_ctx, codec_parameters.as_ref());
                    ffi::avcodec_open2(avc_ctx, local_codec.as_ref(), std::ptr::null_mut());
                }

                let packet: &mut ffi::AVPacket = unsafe {
                    ffi::av_packet_alloc().as_mut()
                }.expect("not null");

                let mut frame = AVFrame::new().unwrap_or_else(|error| {
                    panic!("Could not create input frame: {:?}", error)
                });

                let mut i = 0;

                println!("Time: {:#?}", before.elapsed().unwrap());
                before = std::time::SystemTime::now();

                let mut sws_context: *mut ffi::SwsContext = std::ptr::null_mut();

                while unsafe { ffi::av_read_frame(avformat_context.raw(), packet) } >= 0 && i < 10 {
                    if packet.stream_index == stream.index() {
                        unsafe {
                            ffi::avcodec_send_packet(avc_ctx, packet);
                        }

                        while unsafe { ffi::avcodec_receive_frame(avc_ctx, frame.as_mut()) } >= 0 {
                            println!(
                                "Frame {}: {:?} @ {}",
                                frame.coded_picture_number(),
                                stream.timestamp(frame.pts()),
                                frame.key_frame()
                            );
                            println!("Reading Time: {:#?}", before.elapsed().unwrap());
                            before = std::time::SystemTime::now();

                            if sws_context.is_null() {
                                sws_context = unsafe {
                                    ffi::sws_getContext(
                                        frame.width(),
                                        frame.height(),
                                        frame.format() as ffi::AVPixelFormat,
                                        160,
                                        90,
                                        ffi::AVPixelFormat_AV_PIX_FMT_RGB24,
                                        ffi::SWS_FAST_BILINEAR as i32,
                                        std::ptr::null_mut(),
                                        std::ptr::null_mut(),
                                        std::ptr::null(),
                                    ).as_mut()
                                }.expect("not null");
                            }

                            let success = unsafe {
                                ffi::sws_scale(
                                    sws_context,
                                    frame.data_ptr(),
                                    frame.linesize().as_ptr(),
                                    0,
                                    frame.height(),
                                    output_frame.data_mut_ptr(),
                                    output_frame.linesize().as_ptr(),
                                )
                            };

                            println!("success: {}", success);
                            println!("Processing Time: {:#?}", before.elapsed().unwrap());
                            before = std::time::SystemTime::now();

                            if success > 0 {
                                image::save_buffer(
                                    format!("/home/janne/Workspace/justflix/data/test/image_{}.png", i),
                                    output_frame.data(0),
                                    output_frame.width() as u32,
                                    output_frame.height() as u32,
                                    image::ColorType::Rgb8,
                                ).unwrap();

                                i += 1;
                            }

                            println!("Writing Time: {:#?}", before.elapsed().unwrap());
                            before = std::time::SystemTime::now();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
