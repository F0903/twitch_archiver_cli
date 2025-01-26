[![Rust Stable](https://github.com/F0903/twitch_archiver_cli/actions/workflows/rust.yml/badge.svg)](https://github.com/F0903/twitch_archiver_cli/actions/workflows/rust.yml)

# twitch_archiver_cli

A fast and tiny downloader for Twitch VODs (also sub-only).
Simply pass in a url, and give it an optional output path with an extension of your choosing, which the video will then be converted to.

To use as a library, see the [twitch_archiver repo.](https://github.com/F0903/twitch_archiver)

## Usage

Either run the executable normally as an interactive cli, or start the exe with parameters.
To download VODs, you must get your client id, as described below

### Hardware Acceleration

For faster conversion, you can use hardware acceleration arguments provided to FFmpeg.
For Nvidia GPUs, you can use the following arguments provided to **get**:
get --input-args "-hwaccel cuda" --output-args "-c:v h264_nvenc" **vod_url**
OR use get --nvenc **vod_url** as a shorthand

### Getting your Client ID

1. Open any Twitch VOD
2. Press F12 to open the dev tools and go to the Network tab. It should now be recording all network requests to and from Twitch
3. Press CTRL+R to reload the page and start from scratch.
4. When the page starts loading, wait until the video starts playing. Then press the red button to stop recording, and scroll to the top of the list.
5. Scroll down through the list until you find a request called "gql".
6. Open this request and find the "Client-Id" header under "Request Headers". Copy the value.

You can now provide the value to the program either with **settings client-id set** or with the _--auth_ option on **get**.

### Downloading sub-only VODs

To download subscriber-only VODs, you need to provide an OAuth token either by setting it through **settings token set** or by passing it to the **get** command with the _--auth_ option.

To get your OAuth token, follow the method on getting your client id above, but copy the value of the "Authorization" header instead, without the first "OAuth" part.

### Library Usage

See the [library repo.](https://github.com/F0903/twitch_archiver)

## Commands

get **url** _options_

> Downloads specified VOD.  
> Example: **get <https://www.twitch.tv/videos/1199379108> -o vod.mp4**
>
> _options:_
>
> - --auth **token**
> - --client-id **id**
> - --input_args **ffmpeg_input_args**
> - --output_args **ffmpeg_output_args**
> - -o **output_path**

settings **sub-command** **operator** token set **auth_token**

> Command for handling settings. Values saved in settings.json.
>
> - token get  
>   Gets the current token.
>
> - token set **token**  
>   Sets the current token.
>
> - client-id get  
>   Gets the curret client-id
>
> - client-id set **id**  
>   Sets the current client-id

version

> Gets the current version of the program.
