#include <stdio.h>
#include <libxml/parser.h>
#include <libxml/xmlschemas.h>

// Error handler function
void schemaErrorHandler(void *ctx, const char *msg, ...)
{
    va_list args;
    va_start(args, msg);
    fprintf(stderr, "Schema Error: ");
    vfprintf(stderr, msg, args);
    va_end(args);
}

// Warning handler function
void schemaWarningHandler(void *ctx, const char *msg, ...)
{
    va_list args;
    va_start(args, msg);
    fprintf(stderr, "Schema Warning: ");
    vfprintf(stderr, msg, args);
    va_end(args);
}

int validateXSDSchema(const char *schemaFile)
{
    xmlSchemaParserCtxtPtr parserCtxt = NULL;
    xmlSchemaPtr schema = NULL;
    int result = 0;

    // Initialize libxml2
    xmlInitParser();
    LIBXML_TEST_VERSION;

    // Create schema parser context
    parserCtxt = xmlSchemaNewParserCtxt(schemaFile);
    if (parserCtxt == NULL)
    {
        fprintf(stderr, "Failed to create schema parser context\n");
        result = -1;
        goto cleanup;
    }

    // Set error and warning handlers
    xmlSchemaSetParserErrors(parserCtxt,
                             (xmlSchemaValidityErrorFunc)schemaErrorHandler,
                             (xmlSchemaValidityWarningFunc)schemaWarningHandler,
                             NULL);

    // Parse the schema
    schema = xmlSchemaParse(parserCtxt);
    if (schema == NULL)
    {
        fprintf(stderr, "Schema parsing failed: %s is not a valid XSD schema\n", schemaFile);
        result = -1;
        goto cleanup;
    }

    printf("Schema validation successful: %s is a valid XSD schema\n", schemaFile);
    result = 0;

cleanup:
    // Clean up
    if (schema != NULL)
    {
        xmlSchemaFree(schema);
    }
    if (parserCtxt != NULL)
    {
        xmlSchemaFreeParserCtxt(parserCtxt);
    }

    // Cleanup libxml2
    xmlSchemaCleanupTypes();
    xmlCleanupParser();
    xmlMemoryDump();

    return result;
}

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        fprintf(stderr, "Usage: %s <schema_file.xsd>\n", argv[0]);
        return 1;
    }

    int result = validateXSDSchema(argv[1]);

    if (result == 0)
    {
        printf("XSD schema is valid!\n");
        return 0;
    }
    else
    {
        printf("XSD schema validation failed!\n");
        return 1;
    }
}
