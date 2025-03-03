import axios from "axios";
import { useEffect, useMemo, useState } from "react";
import { useParams } from "react-router";
import { User } from "../../entity";

export function UserViewPage() {
  const { id } = useParams();
  const [user, setUser] = useState<User | null>(null);

  const fullName = useMemo(
    () => `${user?.family_name} ${user?.first_name}`,
    [user?.first_name, user?.family_name],
  );

  useEffect(
    function () {
      if (id !== undefined) {
        axios
          .get(`/api/users/${id}`, {
            headers: { "Content-Type": "application/json" },
            data: {},
          })
          .then(function (res) {
            setUser(res.data.data.user);
          })
          .catch(function (err) {
            console.error(err);
          });
      }
    },
    [id],
  );

  return (
    <>
      <h2>User detail</h2>
      {user && (
        <ul>
          <li>User name</li>
          <li>{fullName}</li>
          <li>Email</li>
          <li>{user?.email}</li>
        </ul>
      )}
    </>
  );
}
