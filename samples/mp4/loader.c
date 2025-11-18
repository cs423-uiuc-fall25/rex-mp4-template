#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <librex.h>
#include <bpf/libbpf.h>

#define EXE "./target/x86_64-unknown-none/release/mp1"

#define USAGE "./loader [pids...]\n"

// static int process_ringbuf_data(void *, void *data, size_t) {
//     fprintf(stdout, "%s", (char *)data);
// 	   return 0;
// }

int main()
{
	return 0;
}
