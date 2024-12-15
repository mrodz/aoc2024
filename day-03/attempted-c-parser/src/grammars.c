#include <grammars.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>

static inline bool is_digit(char c)
{
	return c >= '0' && c <= '9';
}

static inline bool is_grammar_token(grammar_token_type_t legal_parses, grammar_token_type_t token_type)
{
	return (legal_parses & token_type) == token_type;
}

static inline void clear_bit(grammar_token_type_t *legal_parses, grammar_token_type_t token_type)
{
	*legal_parses &= ~token_type;
}

grammar_parse_code_t grammar_parse(char *buffer)
{
	grammar_t node;
	grammar_token_type_t legal_parses;
	consumer_dyn_fn next_parse;

	grammar_parse_file_t(&buffer, &node, &legal_parses);

	while (true) {
		char *buffer_clone = buffer;
		grammar_t maybe_out;
		grammar_token_type_t maybe_legal_parses;
		while (grammar_parse_from_legal_parses(&legal_parses, &next_parse)) {
			grammar_parse_code_t code = next_parse(&buffer_clone, &maybe_out, &maybe_legal_parses);
			if (code == GRAMMAR_OK) {
				node = 
			}
		}
	}

	while (true) {
		grammar_parse_code_t code = next_parse(&buffer, &node, &legal_parses);
		if (code == GRAMMAR_ERR) {
			return code;
		}

	}




}

grammar_parse_code_t grammar_drop(grammar_t **grammar)
{
	if (grammar == NULL || *grammar == NULL)
		return GRAMMAR_ERR;

	switch ((*grammar)->token_type)
	{
	case INT_TOKEN:
		return grammar_drop_int32_t(grammar);
	case FILE_TOKEN:
		return grammar_drop_file_t(grammar);
	default:
		return GRAMMAR_ERR;
	}
}

static bool grammar_parse_from_legal_parses(grammar_token_type_t *legal_parses, consumer_dyn_fn *out)
{
	if (is_grammar_token(*legal_parses, INT_TOKEN))
	{
		*out = &grammar_parse_int32_t;
		clear_bit(legal_parses, INT_TOKEN);
		return true;
	}

	if (is_grammar_token(*legal_parses, FILE_TOKEN))
	{
		*out = &grammar_parse_file_t;
		clear_bit(legal_parses, FILE_TOKEN);
		return true;
	}

	return false;
}

static grammar_parse_code_t grammar_drop_file_t(grammar_t **self)
{
	*self = NULL;
	return GRAMMAR_OK;
}

static grammar_parse_code_t grammar_parse_file_t(char **buffer, grammar_t *out, grammar_token_type_t *legal_parses)
{
	*out = (grammar_t){.consume = &grammar_parse_file_t, .drop = &grammar_drop_file_t, .grammar_definition = NULL, .token_type = FILE_TOKEN};
	*legal_parses = OP_TOKEN;
	return GRAMMAR_OK;
}

static grammar_parse_code_t grammar_drop_int32_t(grammar_t **self)
{
	void *atol_p_raw = (*self)->grammar_definition;
	long long *atol_p = (long long *)atol_p_raw;
	free(atol_p);
	(*self)->grammar_definition = NULL;
	*self = NULL;
	return GRAMMAR_OK;
}

static grammar_parse_code_t grammar_parse_int32_t(char **buffer, grammar_t *out, grammar_token_type_t *legal_parses)
{
	char *buffer_ll_end_p = *buffer;

	if (!is_digit(*buffer_ll_end_p))
	{
		return GRAMMAR_REJECT;
	}

	do
	{
		buffer_ll_end_p++;
	} while (is_digit(*buffer_ll_end_p));

	long long atol = strtoll(*buffer, &buffer_ll_end_p, 10);

	long long *atol_p = (long long *)malloc(sizeof(long long));
	*atol_p = atol;

	*out = (grammar_t){.consume = &grammar_parse_int32_t, .drop = &grammar_drop_int32_t, .grammar_definition = atol_p, .token_type = INT_TOKEN};
	*buffer = buffer_ll_end_p;
	*legal_parses = COMMA_TOKEN | INT_TOKEN | RPAR_TOKEN;

	return GRAMMAR_OK;
}