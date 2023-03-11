import qrcode
import urllib.parse

# Define the data to be encoded in the QR code (in this case, a dynamic URL)
url = 'https://www.provenance.org/'

# Encode the URL with urllib.parse.quote() to ensure that special characters are properly encoded
url_encoded = urllib.parse.quote(url, safe='')

# Create a QR code instance and add the data
qr = qrcode.QRCode(version=1, box_size=10, border=4)
qr.add_data(url_encoded)

# Compile the QR code and save it as an image file
qr.make(fit=True)
img = qr.make_image(fill_color="black", back_color="white")
img.save("dynamic_qr.png")
