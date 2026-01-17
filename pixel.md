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
