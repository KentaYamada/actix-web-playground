import axios from "axios";
import { useCallback, useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import { Title } from "@mantine/core";
import { DefaultLayout, UserCard } from "@components";
import { User, ApiResponseError } from "@entity";

export function UserViewPage() {
  const navigate = useNavigate();
  const { id } = useParams();
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
      axios
        .get(`/api/users/${id}`, {
          headers: { "Content-Type": "application/json" },
          data: {},
        })
        .then((res) => {
          setUser(res.data.user);
        })
        .catch((err: ApiResponseError) => {
          if (err.status === 404) {
            navigate("/notfound");
          }
        });
    }
  }, [id, navigate]);

  return (
    <DefaultLayout>
      <Title order={2}>User detail</Title>
      <UserCard user={user} onNavigateToEditPage={handleNavigateToEditPage} />
    </DefaultLayout>
  );
}
