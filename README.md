# XPlane Normal-Metallic Converter
Nondestructively combines the standard three material maps of PBR into a **new** single map used by XPlane 11.

A simple command line tool that combines &hellip;
1. RGB or RGBA UV Normal PNG
1. BW Metallic PNG
1. BW Roughness PNG  

&hellip; into the special image format, Normal/Metal, that XPlane 11 expects.
- R Channel - UV Map
- G Channel - UV Map
- B Channel - Metallic Map
- A Channel - Roughness Map  

## Usage
1. Create a folder that holds the three maps
1. Create a `config.txt`
    - List each file name with extension ending in a line break
    ```text
    uv_norm.png
    uv_metal.png
    uv_rough.png
    ```
1. Run the program from the command line
    - Feed the folder path into the program

The program will add a new png file into the folder that can be used for PBR workflows in XPlane 11.

## History

I created this little utility since manually creating this file is a pain in Gimp and not everyone has Photoshop. Currently it works by only preserving the Red and Green channels of the UV Normal map, which means the information on the B channel is lost.  

Further improvements under consideration:
- Investigating options for converting a three channel normal into a two channel normal to preserve information.