# Dashboard UI Demo Structure

## Design System
- **Color Palette**: 
  - Background: `#0B0E14` (Deep OLED)
  - Surface: `#161B22` (Soft Dark)
  - Accent: `#58A6FF` (Tech Blue) / `#7EE787` (Success Green)
  - Text: `#C9D1D9` (Primary), `#8B949E` (Secondary)
- **Styling**: Tailwind CSS
- **Icons**: Lucide React

## Component Breakdown
1. **Layouts**
   - `MainLayout.tsx`: Wrapper with Sidebar and Topbar.
2. **Components/Dashboard**
   - `StatCard.tsx`: Small data widgets (Value, Trend, Label).
   - `ChartContainer.tsx`: Wrapper for visualization (using Recharts).
   - `ActivityTable.tsx`: Recent transactions/records list.
3. **Navigation**
   - `Sidebar.tsx`: Collapsible nav with active states.
   - `TopBar.tsx`: Search, Notifications, User Profile.
