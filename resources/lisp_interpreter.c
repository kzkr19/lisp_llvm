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
    TYPE_FUNC,
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

typedef struct _Func
{
    char *name;
    LispData *((*f)(LispData *));
} Func;

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
    else if (type == TYPE_FUNC)
        return "procedure";
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
    else if (data->type == TYPE_FUNC)
        return sizeof(Func);
    else
    {
        printf("runtime error: unkown data type found\n");
        exit(1);
    }
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
    if (size == 0)
        return create_nil();

    LispData *ret = create_cons();
    LispData *cursor = ret;

    for (int i = 0; i < size; i++)
    {
        LispData *temp = (i == (size - 1)) ? create_nil() : create_cons();
        ((Cons *)(cursor->data))->car = xs[i];
        ((Cons *)(cursor->data))->cdr = temp;
        cursor = temp;
    }

    return ret;
}

LispData *create_func(LispData *(*f)(LispData *), const char *name)
{
    LispData *val0 = (LispData *)malloc(sizeof(LispData));
    Func *data = (Func *)malloc(sizeof(Func));
    data->f = f;
    data->name = (char *)malloc((strlen(name) + 1) * sizeof(char));
    strcpy(data->name, name);

    val0->type = TYPE_FUNC;
    val0->data = (void *)data;

    return val0;
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

    while (cursor->type != TYPE_NIL)
    {
        if (cursor->type == TYPE_CONS)
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

// or
LispData *lisp_bool_or(LispData *args)
{
    size_t len = list_length(args);
    bool f = false;

    for (int i = 0; i < len; i++)
    {
        LispData *s = fetch_nth_arg(args, i);

        if (s->type != TYPE_BOOL)
        {
            printf("runtime error: "
                   "invalid type of argument"
                   "(all element should be bool in or, but we got %s)",
                   type_to_string(s->type));
            exit(0);
        }
        f |= *(bool *)(s->data);
    }

    return create_bool(f);
}

void display_core(LispData *data, bool start)
{
    if (data->type == TYPE_INTEGER)
        printf("%" PRId64 "", *(int64_t *)(data->data));
    else if (data->type == TYPE_STRING)
        printf("%s", (char *)data->data);
    else if (data->type == TYPE_CHAR)
        printf("%c", *(char *)data->data);
    else if (data->type == TYPE_NIL)
        printf("%s", start ? "()" : ")");
    else if (data->type == TYPE_BOOL)
        printf("%s", *(bool *)(data->data) ? "#t" : "#f");
    else if (data->type == TYPE_CONS)
    {
        Cons *cons = (Cons *)(data->data);
        bool pair = cons->cdr->type != TYPE_NIL && cons->cdr->type != TYPE_CONS;
        if (start || pair)
            printf("(");
        display_core(cons->car, true);
        if (pair)
            printf(". ");
        else if (cons->cdr->type != TYPE_NIL)
            printf(" ");
        display_core(cons->cdr, false);
    }
    else if (data->type == TYPE_FUNC)
        printf("#<procedure:%s>", ((Func *)data->data)->name);
    else
    {
        printf("runtime error: unknown data type");
        exit(0);
    }
}

LispData *lisp_display(LispData *args)
{
    size_t len = list_length(args);
    if (len != 1)
    {
        printf("runtime error: "
               "invalid number of argument"
               "(we need exactly 1, we got %zu in function display)",
               len);
        exit(0);
    }

    LispData *data = fetch_nth_arg(args, 0);
    display_core(data, true);

    return create_bool(false);
}

int main()
{
    LispData *v1 = create_integer(1);
    lisp_display(create_list((LispData *[]){v1}, 1));
    LispData *v2 = create_integer(2);
    lisp_display(create_list((LispData *[]){v2}, 1));
    LispData *xs = create_list((LispData *[]){v1, v2}, 2);
    lisp_display(create_list((LispData *[]){xs}, 1));
    LispData *f = lisp_number_equal(xs);
    lisp_display(create_list((LispData *[]){f}, 1));
    LispData *g = lisp_number_add(xs);
    lisp_display(create_list((LispData *[]){g}, 1));
    LispData *h = lisp_number_sub(xs);
    lisp_display(create_list((LispData *[]){h}, 1));
    LispData *i = create_func(lisp_display, "display");
    lisp_display(create_list((LispData *[]){i}, 1));
    return 0;
}