Sem descodificador H.264 o WebKitGTK abre o MP4 mas não consegue descodificá-lo — daí o`qtdemux ... Got data flow before stream-start` e o vídeo parado. Nada disto se resolve com código: o WebKitGTK (usado pelo Tauri no Linux) delega a descodificação ao GStreamer do sistema.

sudo apt install -y gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav

gst-inspect-1.0 | grep -iE "avdec_h264|openh264|h264parse"
gst-launch-1.0 playbin uri=https://videos.pexels.com/video-files/34476228/14608814_3840_2160_60fps.mp4