#include <stdio.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdlib.h>
#include <malloc.h>
#include <string.h>

typedef enum _LispType
{
    TYPE_INTEGER = 1,
    TYPE_STRING,
    TYPE_CHAR,
    TYPE_NIL,
    TYPE_BOOL,
    TYPE_CONS,
} LispType;

typedef struct _Nil
{
    uint8_t dammy;
} Nil;

typedef struct _LispData
{
    LispType type;
    void *data;
} LispData;

typedef struct _Cons
{
    LispData *car;
    LispData *cdr;
} Cons;

typedef struct _Range
{
    size_t start;
    size_t end;
} Range;

const char *type_to_string(LispType type)
{
    if (type == TYPE_INTEGER)
        return "integer";
    else if (type == TYPE_STRING)
        return "string";
    else if (type == TYPE_CHAR)
        return "char";
    else if (type == TYPE_NIL)
        return "nil";
    else if (type == TYPE_BOOL)
        return "bool";
    else if (type == TYPE_CONS)
        return "conscell";
    else
        return "unkown_type";
}

size_t get_memory_size(LispData *data)
{
    if (data->type == TYPE_INTEGER)
        return sizeof(int64_t);
    else if (data->type == TYPE_STRING)
        return strlen(data->data) + 1;
    else if (data->type == TYPE_CHAR)
        return sizeof(char);
    else if (data->type == TYPE_NIL)
        return sizeof(Nil);
    else if (data->type == TYPE_BOOL)
        return sizeof(bool);
    else if (data->type == TYPE_CONS)
        return sizeof(Cons);
    else
    {
        printf("runtime error: unkown data type found\n");
        exit(1);
    }
}

char *data_to_string(LispData *data)
{
    size_t s = get_memory_size(data);
    char *buff = (char *)malloc((2 * s + 32) * sizeof(char)); // TODO:free buffer...
    const char *typename = type_to_string(data->type);

    if (data->type == TYPE_INTEGER)
    {
        sprintf(buff, "LispData{type=%s, data=%d}", typename, *(int *)(data->data));
    }
    else if (data->type == TYPE_STRING)
    {
        // TODO: implement later
        sprintf(buff, "LispData{type=%s, data=xxx}", typename);
    }
    else if (data->type == TYPE_CHAR)
    {
        sprintf(buff, "LispData{type=%s, data='%c'}", typename, *(char *)(data->data));
    }
    else if (data->type == TYPE_NIL)
    {
        sprintf(buff, "LispData{type=%s}", typename);
    }
    else if (data->type == TYPE_BOOL)
    {
        char *t_or_f = *(bool *)(data->data) ? "#t" : "#f";
        sprintf(buff, "LispData{type=%s, data=%s}", typename, t_or_f);
    }
    else if (data->type == TYPE_CONS)
    {
        // TODO: display inside?
        Cons *c = data->data;
        sprintf(buff, "LispData{type=%s, data={%p,%p}}", typename, (void *)c->car, (void *)c->cdr);
    }
    else
    {
        sprintf(buff, "LispData{type=unknown}");
    }

    return buff;
}

LispData *create_integer(const int64_t n)
{
    int64_t *val0_data = (int64_t *)malloc(sizeof(int64_t));
    *val0_data = n;
    LispData *val0 = (LispData *)malloc(sizeof(LispData));

    val0->type = TYPE_INTEGER;
    val0->data = (void *)val0_data;

    return val0;
}

LispData *create_bool(const bool f)
{
    bool *val0_data = (bool *)malloc(sizeof(bool));
    *val0_data = f;
    LispData *val0 = (LispData *)malloc(sizeof(LispData));

    val0->type = TYPE_BOOL;
    val0->data = (void *)val0_data;

    return val0;
}

LispData *create_cons()
{
    Cons *data = (Cons *)malloc(sizeof(Cons));
    LispData *val0 = (LispData *)malloc(sizeof(LispData));

    val0->type = TYPE_CONS;
    val0->data = (void *)data;

    return val0;
}

LispData *create_nil()
{
    Nil *data = (Nil *)malloc(sizeof(Nil));
    LispData *val0 = (LispData *)malloc(sizeof(LispData));

    val0->type = TYPE_NIL;
    val0->data = (void *)data;

    return val0;
}

LispData *create_list(LispData *xs[], size_t size)
{
    LispData *ret = create_cons();
    LispData *cursor = ret;

    for (int i = 0; i < size; i++)
    {
        LispData *temp = (i == size - 1) ? create_nil() : create_cons();
        ((Cons *)(cursor->data))->car = xs[i];
        ((Cons *)(cursor->data))->cdr = temp;
    }

    return ret;
}

LispData *copy_data(LispData *original)
{
    size_t size = get_memory_size(original);
    LispData *d = (LispData *)malloc(sizeof(LispData));
    d->data = malloc(size);

    memcpy(d->data, original->data, size);

    return d;
}

int list_length(LispData *data)
{
    LispData *cursor = data;
    size_t count = 0;

    while (true)
    {
        if (cursor->type == TYPE_NIL)
        {
            break;
        }
        else if (cursor->type == TYPE_CONS)
        {
            Cons *temp = (Cons *)cursor->data;
            cursor = temp->cdr;
            count++;
        }
        else
        {
            printf(
                "runtime error: "
                "the argument of list_length is not list"
                "(found type %s in argument)\n",
                type_to_string(cursor->type));
            exit(0);
        }
    }

    return count;
}

LispData *fetch_nth_arg(LispData *data, size_t n)
{
    LispData *cursor = data;
    size_t count = 0;

    while (true)
    {
        if (cursor->type == TYPE_NIL)
        {
            printf("runtime error: "
                   "invalid number of argument"
                   "(try to fetch %zu argument, but we got %zu arguments).\n",
                   n, count);
            exit(0);
        }
        else if (cursor->type == TYPE_CONS)
        {
            Cons *temp = (Cons *)cursor->data;
            if (count == n)
            {
                cursor = temp->car;
                break;
            }
            else
            {
                count++;
                cursor = temp->cdr;
            }
        }
        else
        {
            printf(
                "runtime error: "
                "the function's argument is not list."
                "(found type %s in cdr)\n",
                type_to_string(cursor->type));
            exit(0);
        }
    }

    return cursor;
}

LispData *lisp_car(LispData *args)
{
    size_t len = list_length(args);
    if (len != 1)
    {
        printf("runtime error: "
               "invalid number of argument"
               "(expected 1, we got %zu in function car)",
               len);
        exit(0);
    }
    LispData *list = fetch_nth_arg(args, 0);

    if (list->type == TYPE_CONS)
    {
        return ((Cons *)list->data)->cdr;
    }
    else
    {
        printf(
            "runtime error: "
            "the argument of car is not list"
            "(found type %s)\n",
            type_to_string(list->type));
        exit(0);
    }
}

// =
LispData *lisp_number_equal(LispData *args)
{
    size_t len = list_length(args);
    if (len == 0)
    {
        printf("runtime error: "
               "invalid number of argument"
               "(we need at lest 1, we got 0 in function =)");
        exit(0);
    }
    LispData *first = fetch_nth_arg(args, 0);

    bool flag = true;

    for (int i = 0; i < len; i++)
    {
        LispData *s = fetch_nth_arg(args, i);

        if (s->type != TYPE_INTEGER)
        {
            printf("runtime error: "
                   "invalid type of argument"
                   "(all element should be integer in =, but we got %s)",
                   type_to_string(s->type));
            exit(0);
        }

        flag &= *(int *)(s->data) == *(int *)(first->data);
    }

    return create_bool(flag);
}

// +
LispData *lisp_number_add(LispData *args)
{
    size_t len = list_length(args);
    int64_t n = 0;

    for (int i = 0; i < len; i++)
    {
        LispData *s = fetch_nth_arg(args, i);

        if (s->type != TYPE_INTEGER)
        {
            printf("runtime error: "
                   "invalid type of argument"
                   "(all element should be integer in +, but we got %s)",
                   type_to_string(s->type));
            exit(0);
        }

        n += *(int *)(s->data);
    }

    return create_integer(n);
}

// -
LispData *lisp_number_sub(LispData *args)
{
    size_t len = list_length(args);
    if (len == 0)
    {
        printf("runtime error: "
               "invalid number of argument"
               "(we need at lest 1, we got 0 in function =)");
        exit(0);
    }
    int64_t n = 0;

    for (int i = 0; i < len; i++)
    {
        LispData *s = fetch_nth_arg(args, i);

        if (s->type != TYPE_INTEGER)
        {
            printf("runtime error: "
                   "invalid type of argument"
                   "(all element should be integer in -, but we got %s)",
                   type_to_string(s->type));
            exit(0);
        }
        int temp = *(int *)(s->data);
        n = i == 0 ? temp : n - temp;
    }

    return create_integer(n);
}

int main()
{
    LispData *v1 = create_integer(1);
    printf("%s\n", data_to_string(v1));
    LispData *v2 = create_integer(2);
    printf("%s\n", data_to_string(v2));
    LispData *xs = create_list((LispData *[]){v1, v2}, 2);
    printf("%s\n", data_to_string(xs));
    LispData *f = lisp_number_equal(xs);
    printf("%s\n", data_to_string(f));
    return 0;
}