import React, { ChangeEvent, useState } from "react";

type InputType = "text" | "number";

type ValueType<T extends InputType> = T extends "number" ? number : string;

interface FormFieldProps<T extends InputType> {
  fieldName: string;
  initVal: ValueType<T>;
  onChange: (params: { name: string; value: ValueType<T> }) => void;
  type?: T;
}

export const FormField = <T extends InputType>({
                                                 fieldName,
                                                 initVal,
                                                 onChange,
                                                 type = "text" as T
                                               }: FormFieldProps<T>) => {
  const [value, setValue] = useState<string>(String(initVal));

  const handleOnChange = (e: ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setValue(newValue);

    let parsedValue: ValueType<T>;
    if (type === "number") {
      parsedValue = (newValue === "" ? NaN : Number(newValue)) as ValueType<T>;
    } else {
      parsedValue = newValue as ValueType<T>;
    }

    onChange({ name: fieldName, value: parsedValue });
  };

  return (
    <div className="flex flex-nowrap items-end">
      <label
        className="flex items-end font-spaceMono text-[12px] italic text-white"
        htmlFor={fieldName}
      >
        {fieldName}:
      </label>
      <input
        id={fieldName}
        onChange={handleOnChange}
        className="ml-2 flex h-[15px] items-end border-none bg-transparent font-sequel100Black text-[12px] font-75 leading-[8px] text-white outline-0"
        type={type}
        value={value}
      />
    </div>
  );
};
