cargo run --release
rm output.mp4

# rm output.mp4 if it exists
if [ -f output.mp4 ]; then
    rm output.mp4
fi

ffmpeg -framerate 60 \
       -i output/%d.png \
       -c:v libx264 \
       -profile:v high \
       -crf 18 \
       -pix_fmt yuv420p \
       output.mp4


rm output/*.png
