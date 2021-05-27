#include "stdio.h";

signed int factorial (signed int x,) {
signed int output = 1;
signed int i = 0;
while (i < x) {
output = output * i;
i = i + 1;
}
return output;
}

void main () {
signed int x = 10;
signed int y = 10;
signed int z = x + y;
if (x + y == z) {
z = z + z;
} else {
z = z - z;
}
}
