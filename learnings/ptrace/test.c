// a loop that output incrementing numbers per 1 sec
// Compile with: gcc -o test test.c
// Run with: ./test

#include <stdio.h>
#include <unistd.h>
#include <sys/syscall.h>
#include <time.h>

int main() {
    setbuf(stdout, NULL);  // Disable buffering
    int i = 0;
    while (1) {
        printf("%d\n", i);
        // Force syscall-based sleep
        syscall(SYS_nanosleep, &(struct timespec){ .tv_sec = 1 }, NULL);
        i++;
    }
    return 0;
}