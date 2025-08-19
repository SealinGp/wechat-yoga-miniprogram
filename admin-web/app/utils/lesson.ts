
export interface Teacher {
  id: number;
  name: string;
  description?: string;
  avatar_url?: string;
  bio?: string;
  certifications?: string[];
  specialties?: string[];
  experience_years: number;
  average_rating?: number;
  total_ratings: number;
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

export interface Location {
  id: number;
  name: string;
  description?: string;
  capacity: number;
  equipment?: string[];
  facilities?: string[];
  floor_number?: number;
  room_number?: string;
  is_accessible: boolean;
  booking_enabled: boolean;
  hourly_rate?: number;
  images?: string[];
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

export interface Lesson {
  id: number;
  title: string;
  description?: string;
  teacher?: Teacher;
  location?: Location;
  lesson_type: LessonType;
  difficulty_level: DifficultyLevel;
  start_time: string;
  end_time: string;
  max_students: number;
  current_students: number;
  price?: number;
  equipment_required?: string[];
  prerequisites?: string;
  cancellation_policy?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
  is_active: boolean;
}


export enum LessonType {
  Team = 'team',
  SmallClass = 'small_class',
  Private = 'private',
  EquiqmentSmallClass = 'equipment_small_class',
  Workshop = 'workshop'
}

export enum DifficultyLevel {
  Beginner = 'beginner',
  Intermediate = 'intermediate',
  Advanced = 'advanced',
  AllLevels = 'all_levels'
}

export function getLessonTypeStr(lt: LessonType): string {
  switch (lt) {
    case LessonType.Team:
      return '团课';
    case LessonType.SmallClass:
      return '小班课';
    case LessonType.Private:
      return '私教课';
    case LessonType.EquiqmentSmallClass:
      return '器械小班课';
    case LessonType.Workshop:
      return '工作坊';
    default:
      return '未知课程类型';
  }
}

export function getDifficultyLevelStr(dl: DifficultyLevel): string {
  switch (dl) {
    case DifficultyLevel.Beginner:
      return '初级';
    case DifficultyLevel.Intermediate:
      return '中级';
    case DifficultyLevel.Advanced:
      return '高级';
    case DifficultyLevel.AllLevels:
      return '全水平';
    default:
      return '未知难度';
  }
}