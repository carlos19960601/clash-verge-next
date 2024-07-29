import { getProfiles } from "@/services/cmds";
import useSWR from "swr";

export const useProfiles = () => {
  const { data: profiles, mutate: mutateProfiles } = useSWR("getProfiles", getProfiles);

  return {
    profiles,
  }
}