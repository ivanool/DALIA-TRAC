import React from "react";
import './Sidebar.css';

interface TooltipProps {
  text: string;
  children: React.ReactNode;
  position?: 'right' | 'left' | 'top' | 'bottom';
}

const Tooltip: React.FC<TooltipProps> = ({ text, children, position = 'right' }) => {
  const [visible, setVisible] = React.useState(false);
  return (
    <span
      className="sidebar-tooltip-wrapper"
      onMouseEnter={() => setVisible(true)}
      onMouseLeave={() => setVisible(false)}
      style={{ position: 'relative', display: 'inline-flex' }}
    >
      {children}
      {visible && (
        <span className={`sidebar-tooltip sidebar-tooltip-${position}`}>
          {text}
        </span>
      )}
    </span>
  );
};

export default Tooltip;
