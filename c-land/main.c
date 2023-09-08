#include <stddef.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>

struct Data {
    char* message;
    size_t length;
};
typedef struct Data Data;

void free_cringe_ptr(void*, size_t);

void print_and_transform(Data* data) {
    printf("[C Land] Received message: ");
    for (size_t i = 0; i < data->length; i++) {
        printf("%c", data->message[i]);
    }
    printf("\n");
    printf("[C Land] Copying new message...\n");
    const char* returned_message = "Hello world from C! (and thanks for visiting!)";
    size_t returned_length = strlen(returned_message);
    char* new_ptr = malloc(returned_length * sizeof(char));
    char* old_ptr = data->message;
    data->message = new_ptr;
    printf("[C Land] Freeing cringe ptr...\n");
    free_cringe_ptr(old_ptr, data->length);
    for (size_t i = 0; i < returned_length; i++) {
        data->message[i] = returned_message[i];
    }
    size_t bytes_to_free = data->length - returned_length;
    data->length = returned_length;
    printf("[C Land] Returning hold to Rust...\n");
}