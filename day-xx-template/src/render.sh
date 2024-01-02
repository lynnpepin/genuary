cargo run --release
rm output.mp4

ffmpeg -framerate 60 \
       -i output/%d.png \
       -c:v libx264 \
       -profile:v high \
       -crf 30 \
       -pix_fmt yuv420p \
       output.mp4

rm output/*.png
