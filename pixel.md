alr, so now im gonna try to make the pixels be stored in the most efficient way possibel..
so i wanna store the least amount of pixel as possible, knowing we might have 256x256 is already 16bit, wich can be improved
we can instead, store a vector for each pixels, so like, we have a vec for each row, that mean..
so lets say a row can have 256 cells
a vec can have nothing or have a value saying the position of the pixel, if a pixel have a higher z index we ofc make replace it with the other that had the same value, like that we dont need to store the z index separatly, alr, now lets calulate the worst case ever
we have a Vec<Vec<u8>>
lets say a 256x256
so we have 256 vectors , each with 256 unsigned 8bit integers
so if we suppose that the vec doesnt affect the size
256 * u8 = 2048b
2048 * 256 = 524288b = 65536o =  64Ko
nah, thats already a lot of KB
oh wait uh, i forgot to add the colors :sob:
uhhhh, so i have to add another u8 , that mean well add another 64Ko + a vector of the repeated colors, so i expect around 260Ko
wich is actually not bad, but i think i can do better

alr, so ima use 16 bit colors instead,
1 color = 2 byte

thats how the OR operator works

----|
0000| 
0001|
----|
0001|
----|

RRRRRGGGGGGBBBBB
123456789ABCDEFG
0101
0011
0001

alr alr, so i have to add a Z buffer atleast, so we can know if the pixel has higher or lower z value 
alr, so i have to use the most optimized way, uh, we can use 2 bits for each pixel, hmmmmmm
4 is not enough i think, maybe 4 ? 15 seems as a reasonable amount of z index
here is the final result of Rael (the thing that has all the stuff)
196712 bytes = 196,712 KB  
i just raelised that Rael has only 52 bytes alone, and now a whoping 19712 bytes after adding two screens

# aaaaaaah

alr so abt the images, i already did one, but its so unoptimized, anyways, now ima redo it, so first of all, i need to process the images, to get a 
[[u8; widht]; height] and a Vec<Color>, the files should be preproceded and the files we have to give to it should always have only 255 colors, and we should allow resizing, + we have to make a optimized version of how to add the pixels, a add image function, that chacks if any color in its Vec<Color> is inside the main one, to know the index, and then when adding the pixels, we make sure all of them have the exact color index, that should match the main one, and ofc we check the z index layer to know wheter the pixel should be added or not, the function should look like smt like this:
rael.set_image(path: &str, x: usize: y:usize, z: u8, widht: u16, height: u16, stretch: bool)
the pre proccessed file should be smt like 
pixels: [[u8; widht]; height]
colors: Vec<Color>
where u8 in pixels is the index of the color, and we can ignore the part of the image that is outside 512x512

        

