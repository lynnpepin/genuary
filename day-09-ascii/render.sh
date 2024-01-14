cargo run --release
rm output.mp4

# copy the last output png to ./final.png
cp output/$(ls output | tail -n 1) final.png

# rm output.mp4 if it exists
if [ -f output.mp4 ]; then
    rm output.mp4
fi

ffmpeg -framerate 60 \
       -i output/%d.png \
       -c:v libx264 \
       -profile:v high \
       -crf 30 \
       -pix_fmt yuv420p \
       output.mp4

rm output/*.png
