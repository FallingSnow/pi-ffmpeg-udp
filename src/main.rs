use std::io::Result;
fn main() -> Result<()> {
    let file = std::fs::File::open("/tmp/FPS_test_1080p60_L4.2_444.h264")?;

    // ********
    // Create input stream
    // ********
    let stream = {
        use ac_ffmpeg::format::io::IO;
        let stream = IO::from_read_stream(file);
        stream
    };

    // ********
    // Create Demuxer
    // ********
    let mut demuxer = {
        use ac_ffmpeg::format::demuxer::Demuxer;
        let demuxer = Demuxer::builder()
            .build(stream)
            .expect("Failed to create demuxer")
            .find_stream_info(None)
            .map_err(|(_, err)| err)
            .expect("Failed to get stream info");
        demuxer
    };

    let (stream_index, params) = demuxer
        .codec_parameters()
        .iter()
        .enumerate()
        .find(|(_, params)| params.is_video_codec())
        .expect("No video stream found");

    // ********
    // Create Decoder
    // ********
    let mut decoder = {
        use ac_ffmpeg::codec::video::VideoDecoder;
        let params = params.as_video_codec_parameters().unwrap();

        let decoder = VideoDecoder::from_codec_parameters(params)
            .expect("Unable to get codec paramters")
            .build()
            .expect("Failed to create video decoder");

        decoder
    };

    {
        use ac_ffmpeg::codec::Decoder;
        // process data
        while let Some(packet) = demuxer.take().expect("Failed to demux") {
            if packet.stream_index() != stream_index {
                continue;
            }

            decoder.push(packet).expect("Could not send video packet to decoder");

            while let Some(frame) = decoder.take().expect("Failed to decode frame") {
                println!("{}", frame.pixel_format().name());
                // frame.planes().first().
            }
        }

        decoder.flush().expect("Failed to flush decoder");

        while let Some(frame) = decoder.take().expect("Failed to decode frame") {
            println!("{}", frame.pts().as_f32().unwrap_or(0f32));
        }
    }

    Ok(())
}
