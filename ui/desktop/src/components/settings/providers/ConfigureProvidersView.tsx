import { ScrollArea } from '../../ui/scroll-area';
import BackButton from '../../ui/BackButton';
import { ConfigureProvidersGrid } from './ConfigureProvidersGrid';
import MoreMenuLayout from '../../more_menu/MoreMenuLayout';

export default function ConfigureProvidersView({ onClose }: { onClose: () => void }) {
  return (
    <div className="h-screen w-full">
      <MoreMenuLayout showMenu={false} />

      <ScrollArea className="h-full w-full">
        <div className="px-8 pt-6 pb-4">
          <BackButton onClick={onClose} />
          <h1 className="text-3xl font-medium text-textStandard mt-1">Configure</h1>
        </div>

        <div className=" py-8 pt-[20px]">
          <div className="flex justify-between items-center mb-6 border-b border-borderSubtle px-8">
            <h2 className="text-xl font-medium text-textStandard">Providers</h2>
          </div>

          {/* Content Area */}
          <div className="max-w-5xl pt-4 px-8">
            <div className="relative z-10">
              <ConfigureProvidersGrid />
            </div>
          </div>
        </div>
      </ScrollArea>
    </div>
  );
}
