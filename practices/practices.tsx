import { DeformationPractice } from './deformation'
import { IntroductionPractice } from './introduction'
import { SmoothingPractice } from './smoothing'

export const practices = [
  {
    path: '/introduction',
    label: 'Introduction',
    element: <IntroductionPractice />,
  },
  {
    path: '/smoothing',
    label: 'Smoothing',
    element: <SmoothingPractice />,
  },
  {
    path: '/deformation',
    label: 'Deformation',
    element: <DeformationPractice />,
  },
]
