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

            let mut packet = AVPacket::new().unwrap_or_else(|error| {
                panic!("Could not init temporary packet: {:?}", error)
            });

            let mut frame = AVFrame::new().unwrap_or_else(|error| {
                panic!("Could not create input frame: {:?}", error)
            });

            let mut i = 0;

            println!("Time: {:#?}", before.elapsed().unwrap());
            before = std::time::SystemTime::now();

            let mut scale_context = SwsContext::new();

            while avformat_context.read_frame(&mut packet).is_ok() && i < 16 {
                if packet.stream_index() == stream.index() {
                    codec_context.in_packet(&mut packet).unwrap_or_else(|error| {
                        panic!("Could not load packet: {:?}", error)
                    });
                    while codec_context.out_frame(&mut frame).is_ok() {
                        println!(
                            "Frame {}: {:?} @ {}",
                            frame.coded_picture_number(),
                            stream.timestamp(frame.pts()),
                            frame.key_frame()
                        );
                        println!("Reading Time: {:#?}", before.elapsed().unwrap());
                        before = std::time::SystemTime::now();

                        scale_context.reinit(&frame, &output_frame, SwsScaler::FastBilinear).unwrap_or_else(|error| {
                            panic!("Could not reinit scale context: {:?}", error)
                        });
                        scale_context.scale(&frame, &mut output_frame);

                        println!("Processing Time: {:#?}", before.elapsed().unwrap());
                        before = std::time::SystemTime::now();

                        image::save_buffer(
                            format!("/home/janne/Workspace/justflix/data/test/image_{}.png", i),
                            output_frame.data(0),
                            output_frame.width() as u32,
                            output_frame.height() as u32,
                            image::ColorType::Rgb8,
                        ).unwrap();

                        println!("Writing Time: {:#?}", before.elapsed().unwrap());
                        before = std::time::SystemTime::now();

                        i += 1;
                    }
                }
            }
        }
    }

    Ok(())
}
