from fastapi.testclient import TestClient

from main import app

client = TestClient(app)


def test_sum_positive_numbers():
    response = client.post("/sum", json={"a": 2, "b": 3})
    assert response.status_code == 200
    assert response.json() == {"result": 5}


def test_sum_negative_numbers():
    response = client.post("/sum", json={"a": -1, "b": -4})
    assert response.status_code == 200
    assert response.json() == {"result": -5}


def test_sum_mixed_numbers():
    response = client.post("/sum", json={"a": 10, "b": -3})
    assert response.status_code == 200
    assert response.json() == {"result": 7}


def test_sum_zeros():
    response = client.post("/sum", json={"a": 0, "b": 0})
    assert response.status_code == 200
    assert response.json() == {"result": 0}


def test_sum_invalid_payload():
    response = client.post("/sum", json={"a": "not_a_number", "b": 3})
    assert response.status_code == 422


def test_sum_missing_field():
    response = client.post("/sum", json={"a": 5})
    assert response.status_code == 422
