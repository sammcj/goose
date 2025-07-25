import React, { useState } from 'react';
import { FolderDot } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '../ui/Tooltip';

interface DirSwitcherProps {
  hasMessages?: boolean;
  className?: string;
}

export const DirSwitcher: React.FC<DirSwitcherProps> = ({
  hasMessages = false,
  className = '',
}) => {
  const [isTooltipOpen, setIsTooltipOpen] = useState(false);

  const handleDirectoryChange = async () => {
    if (hasMessages) {
      window.electron.directoryChooser();
    } else {
      window.electron.directoryChooser(true);
    }
  };

  return (
    <TooltipProvider>
      <Tooltip open={isTooltipOpen} onOpenChange={setIsTooltipOpen}>
        <TooltipTrigger asChild>
          <button
            className={`z-[100] hover:cursor-pointer text-text-default/70 hover:text-text-default text-xs flex items-center transition-colors pl-1 [&>svg]:size-4 ${className}`}
            onClick={handleDirectoryChange}
          >
            <FolderDot className="mr-1" size={16} />
            <div className="max-w-[200px] truncate [direction:rtl]">
              {String(window.appConfig.get('GOOSE_WORKING_DIR'))}
            </div>
          </button>
        </TooltipTrigger>
        <TooltipContent side="top">
          {window.appConfig.get('GOOSE_WORKING_DIR') as string}
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};
