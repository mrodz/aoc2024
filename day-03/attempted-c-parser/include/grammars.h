#ifndef GRAMMARS
#define GRAMMARS
#include <stdint.h>
#include <stdbool.h>

typedef enum
{
	NO_TOKEN = 0b00000000,
	MUL_TOKEN = 0b00000001,
	INT_TOKEN = 0b00000010,
	LPAR_TOKEN = 0b00000100,
	RPAR_TOKEN = 0b00001000,
	COMMA_TOKEN = 0b00010000,
	FILE_TOKEN = 0b00100000,
	OP_TOKEN = 0b01000000,
} grammar_token_type_t;

typedef enum
{
	GRAMMAR_ERR,
	GRAMMAR_OK,
	GRAMMAR_REJECT,
	GRAMMAR_EOI,
} grammar_parse_code_t;

typedef grammar_parse_code_t (*consumer_dyn_fn)(char **, struct grammar_t *, grammar_token_type_t *);
typedef grammar_parse_code_t (*grammar_drop_fn)(struct grammar_t **);

typedef struct grammar_t
{
	consumer_dyn_fn consume;
	grammar_drop_fn drop;
	void *grammar_definition;
	grammar_token_type_t token_type;
} grammar_t;

typedef struct grammar_int32_t
{
	int32_t number;
} grammar_int32_t;

grammar_parse_code_t grammar_parse(char *buffer);
grammar_parse_code_t grammar_drop(grammar_t **grammar);

static bool grammar_parse_from_legal_parses(grammar_token_type_t *legal_parses, consumer_dyn_fn *out);

static grammar_parse_code_t grammar_parse_int32_t(char **buffer, grammar_t *out, grammar_token_type_t *legal_parses);
static grammar_parse_code_t grammar_drop_int32_t(grammar_t **self);
static grammar_parse_code_t grammar_parse_file_t(char **buffer, grammar_t *out, grammar_token_type_t *legal_parses);
static grammar_parse_code_t grammar_drop_file_t(grammar_t **self);

#endif // #ifndef GRAMMARS