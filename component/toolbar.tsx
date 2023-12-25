import React from 'react';

interface ToolbarProps {
  pages: string[];
}

const Toolbar: React.FC<ToolbarProps> = ({ pages }) => {
  const handlePageJump = (page: string) => {
    // Logic to handle page jump
    console.log(`Jumping to ${page}`);
  };

  return (
    <div className="toolbar">
      {pages.map((page) => (
        <button key={page} onClick={() => handlePageJump(page)}>{page}</button>
      ))}
    </div>
  );
};

export default Toolbar;