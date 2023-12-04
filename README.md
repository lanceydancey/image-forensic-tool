# Image Mapper

## Background
The purpose of this tool is to batch process a set of images into a forensic timeline. The tool pulls the metadata from the images analyzing creation date and geolocation data from the time the photo was taken. This data is exported in json format to a file labeled `output.json` in case the user wants a copy or to review the photos manually. The file is then read and data is used to add pin markers to a static Google map using the Google Maps API. Not all images possess the required data, so it will skip over those that do not during the mapping process. The map is launched to a static webpage using the Rocket framework and the Tera template engine. The pin markers on the map contain an integer indicating their order chronologically. This allows the user to track the location the photos were taken at over time.  

It's worth noting that if you are exporting photos from a phone to your computer, be sure to send the original size. If you send a reduced sized image it will probably truncate the metadata out. If you are taking photos on your phone and you want the GPS location added, be sure that that setting active in your phone's settings. 

## Obtaining a Google Map API key

- Visit this link to the Google Developer portal
```
https://developers.google.com/maps/third-party-platforms/wordpress/generate-api-key
```
- Scroll down a little and click on the blue button labeled `Get an API Key`

![Google Api Button](/assets/google-map-api.png)
- Follow the process and make note of your API key when you recieve it as it will be used shortly
## Using the tool

- Clone the repository onto your computer. 
```
git clone https://github.com/lanceydancey/image-forensic-tool.git
```
- Change into the directory `image-forensic-tool`
- Create a .env file in this folder. You will need to add the line `GOOGLE_MAPS_API_KEY="your_google_api_key"`, replacing the text in quotations with your own Google Maps API key. For information on how to do this see the section with the header `Obtaining a Google Maps API key`.
- Once you have your .env file added, you will be able to run the program.
- From the terminal, run the command 
```
cargo run "<path_to_the_image_directory"
```
- The path needs to be the absolute path to the image file, and it needs to be in quotations. An example of a correct path would be `"C:\Users\lance\image-forensic-tool\images"`
- Once the program executes, you can open a browser and visit
```
localhost:8000
```
- There you will see the map displayed with the pins marking the geolcation of the photos in chronological order




