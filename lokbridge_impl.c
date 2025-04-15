#include "lokbridge.h"
#include "./headers/LibreOfficeKitInit.h"
#include "./headers/LibreOfficeKit.h"

LibreOfficeKit *initialize(const char *install_path) {
    return lok_init_2(install_path, NULL);
}

void destroy_office(LibreOfficeKit* pThis) {
    pThis->pClass->destroy(pThis);
}

char* get_error(LibreOfficeKit* pThis) {
    return pThis->pClass->getError(pThis);
}

void free_error(LibreOfficeKit* pThis, char* message) {
    pThis->pClass->freeError(message);
}

LibreOfficeKitDocument* document_load(LibreOfficeKit* pThis, const char* pURL) {
    return pThis->pClass->documentLoad(pThis, pURL);
}

void destroy_document(LibreOfficeKitDocument* pThis) {
    pThis->pClass->destroy(pThis);
}

int document_save(LibreOfficeKitDocument* pThis, const char* pUrl, const char* pFormat, const char* pFilterOptions) {
    return pThis->pClass->saveAs(pThis, pUrl, pFormat, pFilterOptions);
}
