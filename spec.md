# One Bit Image

The One Bit Image is a file format to store single bit depth
images, similar to 1-bit BMP files. It is a learning
adventure and does not aim to be ubiquitous or
well-performing. The name is shortened to `obi`, any files
that wish to be identified as this format may use the file
extension `.obi`, for example: `apple.obi`.

## Technical Specification

The file format consists of the following sections:

 - File Header: 10 bytes, 0x0000 - 0x00A0
 - Image Information Header: 16 bytes, 0x00A0 - 0x01A0
 - Pixel data: Variable, 0x1A0 - Variable

### File Header (10 bytes)

This header contains the following metadata about the file
itself:

 - OBI version (2 bytes, 0x0000)
 - File size in bytes (4 bytes, 0x0020)
 - Data offset: Offset from the beginning of the file to the
   beginning of bitmap data (4 bytes, 0x0060)

### Image Information Header (16 bytes)

This header contains metadata about the image:

 - Width: Horizontal size of the bitmap data in pixels (4
   bytes, 0x00A0).
 - Height: Vertical size of the bitmap data in pixels (4
   bytes, 0x00E0).
 - Compression Algorithm: The following types of compression
   are supported: RLE, Kosinki etc. Set 0 if no compression
   (4 bytes, 0x0120).
 - Image size after compression (set 0 if no compression,
   i.e., Compression=0). (4 bytes, 0x0160)

### Pixel Data (ImageInformationHeader.Width * ImageInformationHeader.Height bytes)

The image data. Each pixel is a single bit, the size of
Pixel Data is given by `Width * Height`.
