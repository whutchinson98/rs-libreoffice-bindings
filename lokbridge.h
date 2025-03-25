#ifndef LOK_BRIDGE_H
#define LOK_BRIDGE_H

#include <stddef.h>
#include <stdbool.h>
#include <stdlib.h>
#include "./headers/LibreOfficeKitInit.h"
#include "./headers/LibreOfficeKit.h"

// Function declarations (just prototypes without bodies)
LibreOfficeKit* initialize(const char* install_path);
void destroy_office(LibreOfficeKit* pThis);
char* get_error(LibreOfficeKit* pThis);
void free_error(LibreOfficeKit* pThis, char* message);
LibreOfficeKitDocument* document_load(LibreOfficeKit* pThis, const char* pURL);
void destroy_document(LibreOfficeKitDocument* pThis);
int document_save(LibreOfficeKitDocument* pThis, const char* pUrl, const char* pFormat, const char* pFilterOptions);

#endif /* LOK_BRIDGE_H */
