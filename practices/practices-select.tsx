import { useLocation, useNavigate } from 'react-router'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

import { practices } from './practices'

export function PracticesSelect() {
  const navigate = useNavigate()
  const location = useLocation()

  const onValueChange = (value: string) => {
    navigate(value)
  }

  return (
    <Select value={location.pathname} onValueChange={onValueChange}>
      <SelectTrigger className="w-[180px] absolute top-4 left-4">
        <SelectValue placeholder="Theme" />
      </SelectTrigger>
      <SelectContent>
        {practices.map(practice => (
          <SelectItem key={practice.path} value={practice.path}>
            {practice.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}
