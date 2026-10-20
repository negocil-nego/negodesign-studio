

export type InputType =
    | 'text'
    | 'email'
    | 'password'
    | 'number'
    | 'tel'
    | 'url'
    | 'search'
    | 'date'
    | 'datetime-local'
    | 'month'
    | 'week'
    | 'time';

export type InputProps = {
    type: InputType;
    required?: boolean
    disabled?: boolean
    placeholder?: string
    value?: string
};

export type InputWithoutLabelProps = {
    inputProps: InputProps;
};