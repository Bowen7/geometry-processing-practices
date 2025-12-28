import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useLocation, useNavigate } from "react-router";
import { Practice01 } from "./01";
import { Practice02 } from "./02";

export const practices = [
  {
    path: "/01",
    label: "Practice 01",
    element: <Practice01 />,
  },
  {
    path: "/02",
    label: "Practice 02",
    element: <Practice02 />,
  },
];

export function PracticesSelect() {
  const navigate = useNavigate();
  const location = useLocation();

  const onValueChange = (value: string) => {
    navigate(value);
  };

  return (
    <Select value={location.pathname} onValueChange={onValueChange}>
      <SelectTrigger className="w-[180px] absolute top-4 left-4">
        <SelectValue placeholder="Theme" />
      </SelectTrigger>
      <SelectContent>
        {practices.map((practice) => (
          <SelectItem key={practice.path} value={practice.path}>
            {practice.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
