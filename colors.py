from PIL import Image


def rgb_to_hex(red, green, blue):
    val1 = (r // 16)
    val2 = (r % 16)
    val3 = (g // 16)
    val4 = (g % 16)
    val5 = (b // 16)
    val6 = (b % 16)
    if val1 > 9:
        if val1 == 10:
            val1 = "A"
        elif val1 == 11:
            val1 = "B"
        elif val1 == 12:
            val1 = "C"
        elif val1 == 13:
            val1 = "D"
        elif val1 == 14:
            val1 = "E"
        elif val1 == 15:
            val1 = "F"
    if val2 > 9:
        if val2 == 10:
            val2 = "A"
        elif val2 == 11:
            val2 = "B"
        elif val2 == 12:
            val2 = "C"
        elif val2 == 13:
            val2 = "D"
        elif val2 == 14:
            val2 = "E"
        elif val2 == 15:
            val2 = "F"
    if val3 > 9:
        if val3 == 10:
            val3 = "A"
        elif val3 == 11:
            val3 = "B"
        elif val3 == 12:
            val3 = "C"
        elif val3 == 13:
            val3 = "D"
        elif val3 == 14:
            val3 = "E"
        elif val3 == 15:
            val3 = "F"
    if val4 > 9:
        if val4 == 10:
            val4 = "A"
        elif val4 == 11:
            val4 = "B"
        elif val4 == 12:
            val4 = "C"
        elif val4 == 13:
            val4 = "D"
        elif val4 == 14:
            val4 = "E"
        elif val4 == 15:
            val4 = "F"
    if val5 > 9:
        if val5 == 10:
            val5 = "A"
        elif val5 == 11:
            val5 = "B"
        elif val5 == 12:
            val5 = "C"
        elif val5 == 13:
            val5 = "D"
        elif val5 == 14:
            val5 = "E"
        elif val5 == 15:
            val5 = "F"
    if val6 > 9:
        if val6 == 10:
            val6 = "A"
        elif val6 == 11:
            val6 = "B"
        elif val6 == 12:
            val6 = "C"
        elif val6 == 13:
            val6 = "D"
        elif val6 == 14:
            val6 = "E"
        elif val6 == 15:
            val6 = "F"
    return str(val1) + str(val2) + str(val3) + str(val4) + str(val5) + str(val6)


# Loads Image
img = Image.open("math_game/WorldData/houses/houses.png")
width, height = img.size
pixels = img.load()
allColors = ""
for y in range(height):
    for x in range(width):
        # Grabs color of pixel at coordinate (x, y)
        r = pixels[x, y][0]
        g = pixels[x, y][1]
        b = pixels[x, y][2]
        allColors = allColors + rgb_to_hex(r, g, b)

# open text file
text_file = open("math_game/WorldData/houses/picture.txt", "w+")
# write string to file
text_file.write(allColors)
# close file
text_file.close()
