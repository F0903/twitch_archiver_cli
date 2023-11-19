[![Rust Stable](https://github.com/F0903/twitch_archiver_cli/actions/workflows/rust.yml/badge.svg)](https://github.com/F0903/twitch_archiver_cli/actions/workflows/rust.yml)

# twitch_archiver_cli

A fast and tiny downloader for Twitch VODs (also sub-only).
Simply pass in a url, and give it an optional output path with an extension of your choosing, which the video will then be converted to.

## Usage

Either run the executable normally as an interactive cli, or start the exe with parameters.

### Hardware Acceleration

For faster conversion, you can use hardware acceleration arguments provided to FFmpeg.
For Nvidia GPUs, you can use the following arguments provided to **get**:
get --input_args "-hwaccel cuda" --output_args "-c:v h264_nvenc" **vod_url**

## Library Usage

See the ![library repo.](https://github.com/F0903/twitch_archiver)

### Subscriber-only VODs

To download subscriber-only VODs, you need to provide an OAuth token either by setting it through **auth token set** or by passing it to the **get** command with the --auth option.

To get your OAuth token do the following:

1. Open any Twitch VOD
2. Press F12 to open the dev tools and go to the Network tab. It should now be recording all network requests to and from Twitch
3. Press CTRL+R to reload the page and start from scratch.
4. When the page starts loading, wait until the video starts playing. Then press the red button to stop recording, and scroll to the top of the list.
5. Scroll down through the list until you find a request called "gql".
6. Open this request and find the "Authorization" header under "Request Headers". Copy the value except for the "OAuth" part.

## Commands

get _options_ **url** _optional-output-path_

> Downloads specified VOD.  
> Example: **get https://www.twitch.tv/videos/1199379108 vod.mp4**
>
> _options:_
>
> - --auth **token**
> - --input_args **ffmpeg_input_args**
> - --output_args **ffmpeg_output_args**

auth token set **auth_token**

> Sets the auth token to use in requests. Saved in settings.json.

auth token get

> Gets the current auth token.

version

> Gets the current version of the program.
