# media-ingestion

Tool to automatically extract preview spritesheets, metadata information and thumbnails from video files.
Designed for use in media library management software.

## Usage

```
USAGE:
    media-ingestion [FLAGS] [OPTIONS] <input> <output>

FLAGS:
        --fast-chroma      
        --fast-rounding    
        --fast-scaling     
    -h, --help             Prints help information
    -V, --version          Prints version information

OPTIONS:
        --format <format>                     [default: jpg]
        --frame-interval <frame-interval>     [default: 2]
        --max-size <max-size>                 [default: 240]
        --num-horizontal <num-horizontal>     [default: 5]
        --num-vertical <num-vertical>         [default: 5]
        --quality <quality>                   [default: 90]
        --scaler <scaler>                     [default: area]

ARGS:
    <input>     
    <output>    
```