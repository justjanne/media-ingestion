use ffmpeg_dev::sys as ffi;
use enum_primitive::*;
use std::collections::HashMap;

mod ffmpeg_api;

use ffmpeg_api::*;

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

        // TODO: HERE BE DRAGONS

        let output_frame = unsafe {
            ffi::av_frame_alloc().as_mut()
        }.expect("not null");

        let num_bytes: usize = unsafe {
            ffi::avpicture_get_size(ffi::AVPixelFormat_AV_PIX_FMT_RGB24, 160, 90) as usize
        };

        let output_frame_buffer = unsafe {
            (ffi::av_malloc(num_bytes) as *mut u8).as_ref()
        }.expect("not null");

        unsafe {
            ffi::avpicture_fill(
                output_frame as *mut ffi::AVFrame as *mut ffi::AVPicture,
                output_frame_buffer,
                ffi::AVPixelFormat_AV_PIX_FMT_RGB24,
                160,
                90,
            );
        }

        match codec_parameters.codec_type() {
            AVMediaType::Video => {
                let avc_ctx: &mut ffi::AVCodecContext = unsafe {
                    ffi::avcodec_alloc_context3(local_codec.as_ref()).as_mut()
                }.expect("not null");

                unsafe {
                    ffi::avcodec_parameters_to_context(avc_ctx, codec_parameters.as_ref());
                    ffi::avcodec_open2(avc_ctx, local_codec.as_ref(), std::ptr::null_mut());
                }

                let packet: &mut ffi::AVPacket = unsafe {
                    ffi::av_packet_alloc().as_mut()
                }.expect("not null");

                let frame: &mut ffi::AVFrame = unsafe {
                    ffi::av_frame_alloc().as_mut()
                }.expect("not null");

                avc_ctx.skip_loop_filter = ffi::AVDiscard_AVDISCARD_NONKEY;
                avc_ctx.skip_idct = ffi::AVDiscard_AVDISCARD_NONKEY;
                avc_ctx.skip_frame = ffi::AVDiscard_AVDISCARD_NONKEY;

                let mut i = 0;

                println!("Time: {:#?}", before.elapsed().unwrap());
                before = std::time::SystemTime::now();

                while unsafe { ffi::av_read_frame(avformat_context.raw(), packet) } >= 0 && i < 10 {
                    if packet.stream_index == stream.index() {
                        unsafe {
                            ffi::avcodec_send_packet(avc_ctx, packet);
                        }

                        while unsafe { ffi::avcodec_receive_frame(avc_ctx, frame) } >= 0 {
                            let key_frame = frame.key_frame != 0;
                            let frame_index = frame.coded_picture_number;

                            println!(
                                "Frame {}: {:?} @ {}",
                                frame_index,
                                stream.timestamp(frame.pts as i64),
                                key_frame
                            );
                            println!("Reading Time: {:#?}", before.elapsed().unwrap());
                            before = std::time::SystemTime::now();


                            /*
                            if frame.width == last_width && frame.height == last_height && (frame.format as AVPixelFormat) == last_format {

                            }
                            */
                            let sws_context: &mut ffi::SwsContext = unsafe {
                                ffi::sws_getContext(
                                    frame.width,
                                    frame.height,
                                    frame.format as ffi::AVPixelFormat,
                                    160,
                                    90,
                                    ffi::AVPixelFormat_AV_PIX_FMT_RGB24,
                                    ffi::SWS_FAST_BILINEAR as i32,
                                    std::ptr::null_mut(),
                                    std::ptr::null_mut(),
                                    std::ptr::null(),
                                ).as_mut()
                            }.expect("not null");

                            let success = unsafe {
                                ffi::sws_scale(
                                    sws_context,
                                    frame.data.as_ptr() as *const *const u8,
                                    &frame.linesize[0],
                                    0,
                                    frame.height,
                                    &output_frame.data[0],
                                    &output_frame.linesize[0],
                                )
                            };

                            println!("success: {}, size: {}", success, num_bytes);
                            println!("Processing Time: {:#?}", before.elapsed().unwrap());
                            before = std::time::SystemTime::now();

                            if success > 0 {
                                image::save_buffer(
                                    format!("/home/janne/Workspace/justflix/data/test/image_{}.png", i),
                                    unsafe {
                                        std::slice::from_raw_parts(output_frame.data[0], num_bytes)
                                    },
                                    160,
                                    90,
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
