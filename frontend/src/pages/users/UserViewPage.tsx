import { useCallback, useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import { Title } from "@mantine/core";
import { DefaultLayout, UserCard } from "@components";
import { User, ApiResponseError } from "@entity";
import { useUserApi } from "@hooks";

export function UserViewPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const { getUserApi } = useUserApi();
  const [user, setUser] = useState<User>({
    id: 0,
    family_name: "",
    first_name: "",
    email: "",
    password: "",
  });

  const handleNavigateToEditPage = useCallback(() => {
    navigate(`/users/${user.id}/edit`);
  }, [user.id, navigate]);

  useEffect(() => {
    if (id !== undefined) {
      getUserApi(Number(id))
        .then((res) => {
          setUser(res.data.user);
        })
        .catch((err: ApiResponseError) => {
          if (err.status === 404) {
            navigate("/notfound");
          }
        });
    }
  }, [id, getUserApi, navigate]);

  return (
    <DefaultLayout>
      <Title order={2}>ユーザー情報</Title>
      <UserCard user={user} onNavigateToEditPage={handleNavigateToEditPage} />
    </DefaultLayout>
  );
}
