#include <jni.h>

extern char* hello_from_rust(const char* input, const char* dbpath);
extern void free_string(const char* ptr);

JNIEXPORT jstring JNICALL
Java_com_example_helloapp_MainActivity_helloFromRust(
    JNIEnv* env,
    jobject thiz,
    jstring input,
    jstring dbpath
) {
    const char* input_str = (*env)->GetStringUTFChars(env, input, 0);

    char* result = hello_from_rust(input_str, dbpath);

    (*env)->ReleaseStringUTFChars(env, input, input_str);

    jstring output = (*env)->NewStringUTF(env, result);

    free_string(result);

    return output;
}