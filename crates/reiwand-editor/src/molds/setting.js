import { props } from "reiwand";

export const setting = (value, { params }) => props.get(params.prop) ?? 0;
