
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>


#define RL_BUFSIZE 1024
#define TOK_BUFSIZE 64
#define TOK_DELIM " \t\r\n\a"

/*
 * Builtin function declarations
 */
int sh_cd(char **args);
int sh_help(char **args);
int sh_exit(char **args);

char *builtin_str[] = {
    "cd",
    "help",
    "exit"
};

int (*builtin_func[]) (char **) = {
    &sh_cd,
    &sh_help,
    &sh_exit
};

int num_builtins() {
    return sizeof(builtin_str) / sizeof(char *);
}

/*
 * Builtin function implementations
 */

int sh_cd(char **args) 
{
    if (args[1] == NULL) {
        fprintf(stderr, "Shell: expected argument to \"cd\"\n");
    } else {
        if (chdir(args[1]) != 0) {
            perror("Shell");
        }
    }

    return 1;
}

int sh_help(char **args) 
{
    printf("John Naylor\n");
    printf("Shell heavily influenced by Stephens Brennan's LSH\n");
    printf("The following are builtin:\n");

    for (int i = 0; i < num_builtins(); i++)
        printf("  %s\n", builtin_str[i]);

    printf("Use the `man` command for info on other programs\n");

    return 1;
}

int sh_exit(char **args)
{
    return 0;
}

/*
 * Shell Parsing and Execution
 */

char *read_line(void)
{
    int bufsize = RL_BUFSIZE;
    int position = 0;
    char *buffer = malloc(sizeof(char) * bufsize);
    int c;

    if (!buffer) {
        fprintf(stderr, "Shell: allocation error\n");
        exit(EXIT_FAILURE);
    }

    while (1) {
        c = getchar();

        if (c == EOF || c == '\n') {
            buffer[position] == '\0';
            return buffer;
        } else {
            buffer[position] = c; 
        }

        position++;

        if (position >= bufsize) {
            bufsize += RL_BUFSIZE; 
            buffer = realloc(buffer, bufsize);
            if (!buffer) {
                fprintf(stderr, "Shell: allocation error\n");
                exit(EXIT_FAILURE);
            }
        }
    }
}

char **split_line(char *line)
{
    int bufsize = TOK_BUFSIZE, position = 0;
    char **tokens = malloc(bufsize * sizeof(char*));
    char *token;

    if (!tokens) {
        fprintf(stderr, "Shell: allocation error\n");
        exit(EXIT_FAILURE);
    }

    token = strtok(line, TOK_DELIM);
    while (token != NULL) {
        tokens[position] = token;
        position++;

        if (position >= bufsize) {
            bufsize += TOK_BUFSIZE;
            tokens = realloc(tokens, bufsize * sizeof(char *));
            if (!tokens) {
                fprintf(stderr, "Shell: allocation error\n");
                exit(EXIT_FAILURE);
            }
        }

        token = strtok(NULL, TOK_DELIM);
    }

    tokens[position] = NULL;
    return tokens;
}

int launch(char **args)
{
    pid_t pid, wpid;
    int status;

    pid = fork();
    if (pid == 0) {
        if (execvp(args[0], args) == -1) 
            perror("Shell");
        exit(EXIT_FAILURE);
    } else if (pid < 0) {
        perror("Shell");
    } else {
        do {
            wpid = waitpid(pid, &status, WUNTRACED);
        } while (!WIFEXITED(status) && !WIFSIGNALED(status));
    }

    return 1;
}

int execute(char **args)
{
    if (args[0] == NULL)
        return 1;

    for (int i = 0; i < num_builtins(); i++) {
        if (strcmp(args[0], builtin_str[i]) == 0) {
            return (*builtin_func[i])(args);
        }
    }

    return launch(args);
}

void shell_loop(void)
{
    char *line;
    char **args;
    int status;

    do {
    
        printf("> ");
        line = read_line();
        args = split_line(line);
        status = execute(args);

        free(line);
        free(args);

    } while (status);
}

int main(int argc, char **argv) 
{
    shell_loop();

    return EXIT_SUCCESS;
}

