import dayjs from "dayjs";

interface Props {
  id: string;
  selected: boolean;
  itemData: IProfileItem;
}

export const ProfileItem = (props: Props) => {
  const { selected, itemData } = props;
  const { uid, name = "Profile", extra, updated = 0 } = itemData;
  const hasUrl = !!itemData.url;
  const description = itemData.desc;

  const from = parseUrl(itemData.url);

  return (
    <div className="">
      <div className="ProfileBox">
        <div className="relative">
          <div className="flex justify-start">
            <div className="flex m-auto my-0"></div>
            <h1>{name}</h1>
          </div>
        </div>
        <div className="h-6 flex items-center justify-between">
          {
            <>
              {description ? <h1>{description}</h1> : hasUrl && <h1>{from}</h1>}
              {hasUrl && (
                <h1> {updated > 0 ? dayjs(updated * 1000).fromNow() : ""}</h1>
              )}
            </>
          }
        </div>
      </div>
    </div>
  );
};

function parseUrl(url?: string) {
  if (!url) return "";

  const regex = /https?:\/\/(.+?)\//;
  const result = url.match(regex);
  return result ? result[1] : "local file";
}
