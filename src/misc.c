/* Copyright 2019
 * Ramon Fried <ramon.fried@gmail.com>
 */

#include <ctype.h>
#include <inttypes.h>
#include "bitwise.h"

/* IEC Standard */
#define KiB (1ULL << 10)
#define MiB (1ULL << 20)
#define GiB (1ULL << 30)
#define TiB (1ULL << 40)
#define PiB (1ULL << 50)

/* SI Standard */
#define kB (1000ULL)
#define MB (1000 * kB)
#define GB (1000 * MB)
#define TB (1000 * GB)
#define PB (1000 * TB)

int g_has_color = 1;
int g_width = 0;
bool g_input_avail;
int g_input;

#define RED   "\x1B[31m"
#define GREEN   "\x1B[32m"
#define YEL   "\x1B[33m"
#define BLUE   "\x1B[34m"
#define MAGENTA   "\x1B[35m"
#define CYAN   "\x1B[36m"
#define WHITE   "\x1B[37m"
#define RESET "\x1B[0m"
#define NOTHING ""

char *color_green = NOTHING;
char *color_red = NOTHING;
char *color_blue = NOTHING;
char *color_magenta = NOTHING;
char *color_cyan = NOTHING;
char *color_white = NOTHING;
char *color_reset = NOTHING;

void die(const char *fmt, ...)
{
	va_list args;
	va_start(args, fmt);

	deinit_terminal();
	/* See interactive.c for reasoning */
	/* deinit_readline(); */
	vfprintf(stderr, fmt, args);
	va_end(args);
	exit(EXIT_FAILURE);
}

int base_scanf(const char *buf, int base, uint64_t *value)
{
	int ret = 0;

	switch (base) {
	case 10:
		ret = sscanf(buf, "%" PRIu64, value);
		break;
	case 16:
		ret = sscanf(buf, "%" PRIX64, value);
		break;
	case 8:
		ret = sscanf(buf, "%" PRIo64, value);
		break;
	case 2:
		ret = binary_scanf(buf, value);
		break;
	default:
		fprintf(stderr, "Unknown base\n");
		break;
	}

	if (ret == EOF || !ret) {
		LOG("Couldn't parse number: %s\n", buf);
		return 1;
	}

	return 0;
}

int lltostr(uint64_t val, char *buf, int base)
{
	int rc;

	switch (base) {
	case 10:
		rc = sprintf(buf, "%" PRIu64, val);
		break;
	case 16:
		rc = sprintf(buf, "%" PRIx64, val);
		break;
	case 8:
		rc = sprintf(buf, "%" PRIo64, val);
		break;
	case 2:
	default:
		sprintf(buf, "Not implemeted");
		return -1;
	}

	if (rc < 0)
		LOG("sprintf failed with error: %d\n", rc);

	return rc;
}

int sprintf_type(uint64_t val, char *buf, output_type type)
{
	int i;
	int pos = 0;

	switch (type) {
	case CMD_OUTPUT_DECIMAL:
		sprintf(buf, "Decimal: %" PRIu64, val);
		break;
	case CMD_OUTPUT_HEXADECIMAL:
		sprintf(buf, "Hexadecimal: 0x%" PRIx64, val);
		break;
	case CMD_OUTPUT_OCTAL:
		sprintf(buf, "Octal: 0%" PRIo64, val);
		break;
	case CMD_OUTPUT_BINARY:
		pos = sprintf(buf, "Binary: ");
		for (i = g_width; i > 0; i--) {
			if ((i % 8 == 0) && (i != g_width)) {
				buf[pos] = '|';
				buf[pos + 1] = ' ';
				pos += 2;
			}
			if (val & BIT(i - 1)) {
				buf[pos] = '1';
			}
			else {
				buf[pos] = '0';
			}
			buf[pos + 1] = ' ';
			pos += 2;
		}
		buf[pos-1] = '\0';
		break;

	default:
		break;
	}

	return 0;
}

int sprintf_size(uint64_t val, char *buf, bool si)
{
	int ret;
	double f_val = val;

	if (si) {
		if (val >= PB)
			ret = sprintf(buf, "%.2lf PB", f_val / PB);
		else if (val >= TB)
			ret = sprintf(buf, "%.2lf TB", f_val / TB);
		else if (val >= GB)
			ret = sprintf(buf, "%.2lf GB", f_val / GB);
		else if (val >= MB)
			ret = sprintf(buf, "%.2lf MB", f_val / MB);
		else if (val >= kB)
			ret = sprintf(buf, "%.2lf Kb", f_val / kB);
		else
			ret = sprintf(buf, "%" PRIu64, val);
	} else {
		if (val >= PiB)
			ret = sprintf(buf, "%.2lf PiB", f_val / PiB);
		else if (val >= TiB)
			ret = sprintf(buf, "%.2lf TiB", f_val / TiB);
		else if (val >= GiB)
			ret = sprintf(buf, "%.2lf GiB", f_val / GiB);
		else if (val >= MiB)
			ret = sprintf(buf, "%.2lf MiB", f_val / MiB);
		else if (val >= KiB)
			ret = sprintf(buf, "%.2lf KiB", f_val / KiB);
		else
			ret = sprintf(buf, "%" PRIu64, val);
	}

	return ret;
}
