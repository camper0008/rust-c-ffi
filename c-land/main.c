#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Data {
    char* message;
    size_t length;
};
typedef struct Data Data;

char* realloc_cringe_ptr(void*, size_t, size_t);

void print_and_transform(Data* data)
{
    printf("[C Land] Received message: ");
    for (size_t i = 0; i < data->length; i++) {
        printf("%c", data->message[i]);
    }
    printf("\n");
    const char* returned_message = "Hello world from C!";
    size_t returned_length = strlen(returned_message);
    printf("[C Land] Reallocating cringe ptr...\n");
    data->message
        = realloc_cringe_ptr(data->message, data->length, returned_length);
    printf("[C Land] Copying new message...\n");
    for (size_t i = 0; i < returned_length; i++) {
        data->message[i] = returned_message[i];
    }
    size_t bytes_to_free = data->length - returned_length;
    data->length = returned_length;
    printf("[C Land] Returning hold to Rust...\n");
}
