#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>
#include <stdint.h>
#include <curl/curl.h>

#include "cJSON/cJSON.h"

typedef struct {
    char *content;
    char *username; // NULLABLE
} Message;

typedef struct {
    char *webhook_url;
    char *message_content;
    char *username; // NULLABLE
} ProgramArgs;

static void print_usage(char *prog) {
    fprintf(stderr, "Usage: %s -u webhook-url -w \"content\"\n", prog);
    exit(EXIT_FAILURE);
}

static void parse_args(ProgramArgs *args, int argc, char **argv) {
    int opt;

    // TODO: can I make the n flag optional? `::` didn't work
    while (-1 != (opt = getopt(argc, argv, "u:w:n:"))) {
        switch (opt) {
            case 'u':
                args->webhook_url = strdup(optarg);
                break;
            case 'w':
                args->message_content = strdup(optarg);;
                break;
            case 'n':
                if (optarg != nullptr)
                    args->username = strdup(optarg);
                else
                    printf("No name supplied!\n");
                break;
            default:
                print_usage(argv[0]);
                break;
        }
    }
}

int PostJSON(CURL *curl, const Message *message) {
    if (!message) {
        fprintf(stderr, "Invalid name or value supplied!\n");
        return 1;
    }

    cJSON *root = cJSON_CreateObject();
    if (!root) {
        fprintf(stderr, "Couldn't create JSON root.\n");
        return 1;
    }

    if (!cJSON_AddStringToObject(root, "content", message->content)) {
        fprintf(stderr, "Couldn't add string to JSON!\n");
        return 1;
    }

    if (message->username != nullptr) {
        if (!cJSON_AddStringToObject(root, "username", message->username)) {
            fprintf(stderr, "Couldn't add username to JSON!\n");
            return 1;
        }
    }

    char *json = cJSON_PrintUnformatted(root);
    if (!json) {
        fprintf(stderr, "PrintUnformatted failed\n");
        return 1;
    }

    char agent[1024] = {0};
    snprintf(agent, sizeof(agent), "libcurl/%s", curl_version_info(CURLVERSION_NOW)->version);
    agent[sizeof(agent) - 1] = 0;
    curl_easy_setopt(curl, CURLOPT_USERAGENT, agent);

    struct curl_slist *headers = nullptr;
    headers = curl_slist_append(headers, "Expect:");
    headers = curl_slist_append(headers, "Content-Type: application/json");
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, json);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, -1L);

    const CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        fprintf(stderr, "Couldn't send request!\n");
    }

    return 0;
}

int main(int argc, char *argv[]) {
    ProgramArgs args = {0};
    parse_args(&args, argc, argv);

    CURL *curl = curl_easy_init();

    if (!curl) {
        fprintf(stderr, "Could not create CURL client.\n");
        exit(EXIT_FAILURE);
    }

    curl_easy_setopt(curl, CURLOPT_URL, args.webhook_url);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, "?wait=true");

    Message message = {
        .content = args.message_content,
        .username = nullptr
    };

    if (args.username != nullptr) {
        message.username = args.username;
    }

    PostJSON(curl, &message);

    curl_easy_cleanup(curl);
    curl_global_cleanup();
    return EXIT_SUCCESS;
}