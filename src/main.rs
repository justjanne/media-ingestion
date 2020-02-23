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

        if codec_parameters.codec_type() == AVMediaType::Video {
            let mut codec_context = AVCodecContext::new(&local_codec).unwrap_or_else(|error| {
                panic!("Could not init codec context: {:?}", error)
            });
            codec_context.set_parameters(&codec_parameters);
            codec_context.open(&local_codec);

            codec_context.set_skip_loop_filter(AVDiscard::NonKey);
            codec_context.set_skip_idct(AVDiscard::NonKey);
            codec_context.set_skip_frame(AVDiscard::NonKey);

            // TODO: HERE BE DRAGONS
            let packet: &mut ffi::AVPacket = unsafe {
                ffi::av_packet_alloc().as_mut()
            }.expect("not null");
            // TODO: END DRAGONS

            let mut frame = AVFrame::new().unwrap_or_else(|error| {
                panic!("Could not create input frame: {:?}", error)
            });

            let mut i = 0;

            println!("Time: {:#?}", before.elapsed().unwrap());
            before = std::time::SystemTime::now();

            //TODO: HERE BE DRAGONS
            while unsafe { ffi::av_read_frame(avformat_context.raw(), packet) } >= 0 && i < 10 {
                if packet.stream_index == stream.index() {
                    unsafe { ffi::avcodec_send_packet(codec_context.raw(), packet) };
                    while unsafe { ffi::avcodec_receive_frame(codec_context.raw(), frame.as_mut()) } >= 0 {
                        // TODO: END DRAGONS

                        println!(
                            "Frame {}: {:?} @ {}",
                            frame.coded_picture_number(),
                            stream.timestamp(frame.pts()),
                            frame.key_frame()
                        );
                        println!("Reading Time: {:#?}", before.elapsed().unwrap());
                        before = std::time::SystemTime::now();

                        // TODO: HERE BE DRAGONS
                        let sws_context = unsafe {
                            ffi::sws_getContext(
                                frame.width(),
                                frame.height(),
                                frame.format() as ffi::AVPixelFormat,
                                output_frame.width(),
                                output_frame.height(),
                                output_frame.format() as ffi::AVPixelFormat,
                                ffi::SWS_FAST_BILINEAR as i32,
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                                std::ptr::null(),
                            ).as_mut()
                        }.expect("not null");

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
                        // TODO: END DRAGONS

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
    }

    Ok(())
}
