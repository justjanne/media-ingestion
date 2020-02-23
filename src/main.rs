pub(crate) mod ffmpeg_api;

use crate::ffmpeg_api::enums::*;
use crate::ffmpeg_api::api::*;
use image::{ImageBuffer, RgbImage};

fn main() -> Result<(), std::io::Error> {
    let mut before = std::time::SystemTime::now();

    let output = "/root/spritesheets";
    let input = "/var/lib/data/jellyfin-media/shows/Star Trek: Picard/Season 01/S01E01 - Remembrance.mp4";

    //let input = "/home/janne/Workspace/justflix/data/video.mp4";
    //let output = "/home/janne/Workspace/justflix/data/spritesheets";

    let mut avformat_context = AVFormatContext::new().unwrap_or_else(|error| {
        panic!("Could not allocate a context to process the video: {:?}", error)
    });
    avformat_context.open_input(input).unwrap_or_else(|error| {
        panic!("Could not open video input: {:?}", error)
    });

    let x = 5;
    let y = 5;
    let mut spritesheet: RgbImage = ImageBuffer::new(160 * x, 90 * x);

    let mut stream: AVStream = avformat_context.find_stream(|stream| {
        stream.codec_parameters().codec_type() == AVMediaType::Video
    }).unwrap_or_else(|| {
        panic!("Could not find video stream")
    });

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

        while avformat_context.read_frame(&mut packet).is_ok() {
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

                    let current: RgbImage = ImageBuffer::from_raw(160, 90, output_frame.data(0).to_vec()).unwrap();

                    image::imageops::overlay(
                        &mut spritesheet,
                        &current,
                        (i % x) * 160,
                        ((i / x) % y) * 90,
                    );

                    println!("Writing Time: {:#?}", before.elapsed().unwrap());
                    before = std::time::SystemTime::now();

                    i += 1;

                    if i % (x * y) == 0 {
                        spritesheet.save(format!("{}/spritesheet_{}.png", output, (i / (x * y)) - 1)).unwrap_or_else(|error| {
                            panic!("Could not write spritesheet: {}", error)
                        });
                        spritesheet = ImageBuffer::new(160 * x, 90 * x);
                        println!("Writing Time: {:#?}", before.elapsed().unwrap());
                        before = std::time::SystemTime::now();
                    }
                }
            }
        }

        if i % (x * y) != 0 {
            spritesheet.save(format!("{}/spritesheet_{}.png", output, i / (x * y))).unwrap_or_else(|error| {
                panic!("Could not write spritesheet: {}", error)
            });
            println!("Writing Time: {:#?}", before.elapsed().unwrap());
            before = std::time::SystemTime::now();
        }
    }

    Ok(())
}
