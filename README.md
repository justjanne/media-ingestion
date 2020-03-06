# media-ingestion

Tool to automatically extract preview spritesheets, metadata information and thumbnails from video files.
Designed for use in media library management software.

## Usage

```
USAGE:
    media-ingestion [OPTIONS] <input> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --frame-interval <frame-interval>     [default: 2]
        --max-size <max-size>                 [default: 160]
        --num-horizontal <num-horizontal>     [default: 5]
        --num-vertical <num-vertical>         [default: 5]

ARGS:
    <input>     
    <output>    
```